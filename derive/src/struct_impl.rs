use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Fields, Ident, TypePath};

use crate::{
    attributes::{self, Attributes},
    Backend,
};

#[allow(clippy::too_many_arguments)]
fn generate_ui_field_for_struct(
    ui: &Ident,
    extent: &Ident,
    struct_name: &Ident,
    field_ident: &Option<Ident>,
    field_order: usize,
    total_field_count: usize,
    field_type: &TypePath,
    attributes: &Attributes,
    mutable: bool,
    backend: Backend,
) -> proc_macro2::TokenStream {
    let (field_name, field_ident) = match field_ident {
        Some(s) => {
            if let Some(rename) = attributes.get_rename() {
                (quote! { #rename }, quote! { #s })
            } else {
                (quote! { #s }, quote! { #s })
            }
        }
        None => {
            let index = syn::Index::from(field_order);

            if total_field_count == 1 {
                (quote! { #struct_name }, quote! { #index })
            } else {
                (quote! { #struct_name.#index }, quote! { #index })
            }
        }
    };

    let readonly_override = attributes.has_readonly();
    let mutable = mutable && !readonly_override;

    let field_type_string = field_type
        .path
        .segments
        .last()
        .expect("Couldn't reach the type of the field.")
        .ident
        .to_string();
    let field_type_str = field_type_string.as_ref();
    let is_numeric_primitive = matches!(
        field_type_str,
        "usize"
            | "isize"
            | "u64"
            | "i64"
            | "f64"
            | "f32"
            | "u32"
            | "i32"
            | "u16"
            | "i16"
            | "u8"
            | "i8"
    );
    let format_attribute = attributes.get_format();
    let drag_range_attribute = attributes.get_range();
    let drag_speed_attribute = attributes.get_speed();
    let numeric_primitive_render = if is_numeric_primitive {
        let mut code = quote! {};

        let format_call = if let Some(format) = format_attribute {
            quote! {
                .display_format(#format)
            }
        } else {
            quote! {}
        };

        let range_call = if let Some((min, max)) = drag_range_attribute {
            let min = min
                .map(|s| s.to_token_stream())
                .unwrap_or_else(|| quote! { #field_type::MIN });
            let max = max
                .map(|s| s.to_token_stream())
                .unwrap_or_else(|| quote! { #field_type::MAX });
            quote! {
                .range(#min, #max)
            }
        } else {
            quote! {}
        };

        let speed_call = if let Some(speed) = drag_speed_attribute {
            quote! {
                .speed(#speed)
            }
        } else if matches!(field_type_str, "f32" | "f64") {
            quote! {
                .speed(0.001f32)
            }
        } else {
            quote! {
                .speed(0.2f32)
            }
        };

        if mutable {
            code.extend(quote! {
                let _ = imgui::Drag::new(&format!("{}##{:p}", #field_type_str, std::ptr::addr_of!(self.#field_ident)))
                    #format_call
                    #range_call
                    #speed_call
                    .build(&ui, &mut self.#field_ident);
            });
        } else {
            code.extend(quote! {
                let mut data = self.#field_ident;
                ui.disabled(true, || {
                    let _ = imgui::Drag::new(&format!("{}##{:p}", #field_type_str, std::ptr::addr_of!(self.#field_ident)))
                        #format_call
                        #range_call
                        #speed_call
                        .build(&ui, &mut data);
                });
            });
        }
        code
    } else {
        quote! {}
    };
    // panic!("{numeric_primitive_render}");

    let element_subtree = match backend {
        Backend::Imgui => {
            let ui_element = {
                let mut code = quote! {
                    let _id = #ui.push_id(&format!("{}##{:p}", stringify!(#field_name), std::ptr::addr_of!(self.#field_ident)));
                };

                if mutable {
                    code.extend(
                    if is_numeric_primitive {
                            numeric_primitive_render
                        } else {
                            quote! {
                                (&mut self.#field_ident as &mut dyn imgui_presentable::ImguiPresentable).render_component_mut(#ui, #extent);
                            }
                        });
                } else {
                    code.extend(
                        if is_numeric_primitive {
                            numeric_primitive_render
                        } else {
                            quote! {
                                (&self.#field_ident as &dyn imgui_presentable::ImguiPresentable).render_component(#ui, #extent);
                            }
                        });
                };

                if let Some(text) = attributes.get_tooltip_or_documentation() {
                    let mut tooltip = quote! {
                        {
                            let style = #ui.push_style_color(imgui::StyleColor::Text, [0.5, 0.5, 0.5, 1.0]);
                            #ui.text(#text);
                        }
                    };

                    tooltip.extend(code);
                    code = tooltip;
                }

                code
            };

            quote! {
                let field_name = stringify!(#field_name).replace('"', "");
                // #ui.separator();
                #ui.tree_node_config(&format!("{field_name}##{self:p}"))
                    .opened(true, imgui::Condition::FirstUseEver)
                    .framed(true)
                    .build(|| {
                        #ui_element
                    });
            }
        }
        Backend::Egui => {
            let ui_element = {
                let mut code = quote! {};

                if mutable {
                    code.extend(quote! {
                        (&mut self.#field_ident as &mut dyn imgui_presentable::EguiPresentable).render_component_mut(#ui);
                    });
                } else {
                    code.extend(quote! {
                        (&self.#field_ident as &dyn imgui_presentable::EguiPresentable).render_component(#ui);
                    });
                };

                if let Some(text) = attributes.get_tooltip_or_documentation() {
                    let mut tooltip = quote! {
                        {
                            #ui.label(#text);
                        }
                    };

                    tooltip.extend(code);
                    code = tooltip;
                }

                code
            };

            quote! {
                let field_name = stringify!(#field_name).replace('"', "");
                // #ui.separator();
                #ui.collapsing(field_name, |ui| {
                    #ui_element
                });
            }
        }
    };

    let mut generated = quote! {
        #element_subtree
    };

    #[allow(unused_variables)]
    if let Some(tooltip_text) = attributes.get_tooltip_or_documentation() {
        match backend {
            Backend::Imgui => {
                generated.extend(quote! {
                    if #ui.is_item_hovered() {
                        #ui.tooltip_text(#tooltip_text);
                    }
                });
            }
            Backend::Egui => {
                generated.extend(quote! {
                    // TODO
                });
            }
        }
    }

    generated
}

fn get_type(typ: &syn::Type) -> TypePath {
    match typ {
        syn::Type::Path(path) => path.clone(),
        _ => panic!(
            "Complex types aren't supported yet: {}",
            typ.into_token_stream()
        ),
    }
}

/// Derives the [`ImguiPresentable`] trait for a struct.
pub(crate) fn derive_for_struct(
    derive_input: syn::DeriveInput,
    strukt: syn::DataStruct,
    backends: &[Backend],
) -> proc_macro2::TokenStream {
    let struct_name = derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
    let struct_attributes = match Attributes::parse_many(&derive_input.attrs) {
        Ok(a) => a,
        Err(c) => return c,
    };

    // Validate the arguments:
    if struct_attributes.has_skip() {
        return quote! { compile_error!("Structs shouldn't have the #[imgui_presentation(skip)] attribute.") };
    }

    let fields: Vec<(Option<proc_macro2::Ident>, TypePath, Attributes)> = match strukt.fields {
        Fields::Named(named) => match named
            .named
            .into_iter()
            .map(|f| Attributes::parse_from_field(&f).map(move |a| (a, f)))
            .collect::<attributes::Result<Vec<(Attributes, syn::Field)>>>()
        {
            Ok(a) => a
                .into_iter()
                .map(|(a, f)| (f.ident.clone(), get_type(&f.ty), a))
                .collect(),
            Err(e) => return e,
        },
        Fields::Unnamed(unnamed) => match unnamed
            .unnamed
            .into_iter()
            .map(|f| Attributes::parse_from_field(&f).map(move |a| (a, f)))
            .collect::<attributes::Result<Vec<(Attributes, syn::Field)>>>()
        {
            Ok(a) => a
                .into_iter()
                .map(|(a, f)| (f.ident.clone(), get_type(&f.ty), a))
                .collect(),
            Err(e) => return e,
        },
        _ => return quote! { compile_error!("Unit fields aren't supported.") },
    };

    let total_field_count = fields.len();

    let ui_ident = syn::Ident::new("ui", Span::call_site());
    let extent_ident = syn::Ident::new("extent", Span::call_site());

    let chosen_backend = struct_attributes.get_backends();

    backends
        .iter()
        .filter(|b| {
            if chosen_backend.is_empty() {
                true
            } else {
                chosen_backend.contains(b)
            }
        })
        .fold(quote! {}, |mut implementation, backend| {
            implementation.extend(generate_for_backend(
                &ui_ident,
                &extent_ident,
                &struct_name,
                total_field_count,
                *backend,
                &fields,
                &struct_attributes,
                &impl_generics,
                &ty_generics,
                &where_clause,
            ));
            implementation
        })
}

#[allow(clippy::too_many_arguments)]
fn generate_for_backend(
    ui_ident: &Ident,
    extent_ident: &Ident,
    struct_name: &Ident,
    total_field_count: usize,
    backend: Backend,
    fields: &[(Option<proc_macro2::Ident>, TypePath, Attributes)],
    struct_attributes: &Attributes,
    impl_generics: &syn::ImplGenerics<'_>,
    ty_generics: &syn::TypeGenerics<'_>,
    where_clause: &Option<&syn::WhereClause>,
) -> proc_macro2::TokenStream {
    let ui_elements: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter(|f| !f.2.has_skip())
        .enumerate()
        .map(|(i, f)| {
            generate_ui_field_for_struct(
                ui_ident,
                extent_ident,
                struct_name,
                &f.0,
                i,
                total_field_count,
                &f.1,
                &f.2,
                false,
                backend,
            )
        })
        .collect();

    let ui_elements_mut: Vec<proc_macro2::TokenStream> = {
        if struct_attributes.has_readonly() {
            Vec::default()
        } else {
            fields
                .iter()
                .filter(|f| !f.2.has_skip())
                .enumerate()
                .map(|(i, f)| {
                    generate_ui_field_for_struct(
                        ui_ident,
                        extent_ident,
                        struct_name,
                        &f.0,
                        i,
                        total_field_count,
                        &f.1,
                        &f.2,
                        true,
                        backend,
                    )
                })
                .collect()
        }
    };

    let tooltip = if let Some(text) = struct_attributes.get_tooltip_or_documentation() {
        match backend {
            Backend::Imgui => {
                quote! {
                    {
                        let style = #ui_ident.push_style_color(imgui::StyleColor::Text, [0.5, 0.5, 0.5, 1.0]);
                        #ui_ident.text(#text);
                    }
                }
            }
            Backend::Egui => {
                quote! {
                    {
                        #ui_ident.label(#text);
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let main_menu_items = struct_attributes.get_main_menu_items();
    let has_menu = !main_menu_items.is_empty();

    let mut main_menu_items = struct_attributes
        .get_main_menu_items()
        .into_iter()
        .map(|b| {
            let title = &b.title;
            let method_name = syn::Ident::new(&b.method_name, Span::call_site());
            // TODO figure hot to handle the shortcuts.
            // let hot_key = b.hot_key;

            match backend {
                Backend::Imgui => {
                    quote! {
                        {
                            if #ui_ident.menu_item_config(#title).build() {
                                #[allow(clippy::ignored_unit_patterns)]
                                let _ = self.#method_name();
                            }
                        }
                    }
                }
                Backend::Egui => {
                    quote! {
                        {
                            // TODO
                            // if #ui_ident.button(#title).clicked() {
                            //     #[allow(clippy::ignored_unit_patterns)]
                            //     let _ = self.#method_name();
                            // }
                        }
                    }
                }
            }
        })
        .fold(quote! {}, |mut code, button| {
            code.extend(button);
            code
        });

    if has_menu {
        main_menu_items = match backend {
            Backend::Imgui => quote! {
                if let Some(_token) = #ui_ident.begin_menu_bar() {
                    #main_menu_items
                }
            },
            Backend::Egui => main_menu_items,
        };
    }

    let buttons = struct_attributes
        .get_buttons()
        .into_iter()
        .map(|b| {
            let title = &b.title;
            let method_name = syn::Ident::new(&b.method_name, Span::call_site());

            match backend {
                Backend::Imgui => {
                    quote! {
                        {
                            if #ui_ident.button(#title) {
                                #[allow(clippy::ignored_unit_patterns)]
                                let _ = self.#method_name();
                            }
                        }
                    }
                }
                Backend::Egui => {
                    quote! {
                        {
                            if #ui_ident.button(#title).clicked() {
                                #[allow(clippy::ignored_unit_patterns)]
                                let _ = self.#method_name();
                            }
                        }
                    }
                }
            }
        })
        .fold(quote! {}, |mut code, button| {
            code.extend(button);
            code
        });

    let immutable_render = match backend {
        Backend::Imgui => {
            quote! {
                fn render_component(&self, #ui_ident: &imgui::Ui, #extent_ident: imgui_presentable::Extent) {
                    #tooltip

                    #(#ui_elements;)*
                }
            }
        }
        Backend::Egui => {
            quote! {
                fn render_component(&self, #ui_ident: &mut egui::Ui) {
                    #tooltip

                    #(#ui_elements;)*
                }
            }
        }
    };

    let mutable_render = match backend {
        Backend::Imgui => {
            quote! {
                fn render_component_mut(&mut self, #ui_ident: &imgui::Ui, #extent_ident: imgui_presentable::Extent) {
                    #main_menu_items

                    #tooltip

                    #(#ui_elements_mut;)*

                    #buttons
                }
            }
        }
        Backend::Egui => {
            quote! {
                fn render_component_mut(&mut self, #ui_ident: &mut egui::Ui) {
                    #tooltip

                    #(#ui_elements_mut;)*

                    #buttons
                }
            }
        }
    };

    let trait_name = match backend {
        Backend::Imgui => {
            quote! {
                imgui_presentable::ImguiPresentable
            }
        }
        Backend::Egui => {
            quote! {
                imgui_presentable::EguiPresentable
            }
        }
    };

    let render_window_methods = match backend {
        Backend::Imgui => {
            quote! {
                /// Renders the implementor as a stand-alone window not allowing to
                /// change the values.
                fn render_window(&self, ui: &imgui::Ui, extent: imgui_presentable::Extent) {
                    ui.window(std::any::type_name::<Self>())
                        .resizable(true)
                        .collapsible(true)
                        .bg_alpha(0.7f32)
                        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
                        .menu_bar(#has_menu)
                        .build(|| (self as &dyn imgui_presentable::ImguiPresentable).render_component(ui, extent));
                }

                /// Renders the implementor as a stand-alone window allowing to
                /// change the values.
                fn render_window_mut(&mut self, ui: &imgui::Ui, extent: imgui_presentable::Extent) {
                    ui.window(std::any::type_name::<Self>())
                        .resizable(true)
                        .collapsible(true)
                        .bg_alpha(0.7f32)
                        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
                        .menu_bar(#has_menu)
                        .build(|| (self as &mut dyn imgui_presentable::ImguiPresentable).render_component_mut(ui, extent));
                }
            }
        }
        Backend::Egui => {
            quote! {}
        }
    };

    if struct_attributes.has_readonly() {
        quote! {
            #[doc = "Renders [`Self`] in the immediate gui. The code was automatically generated using the derive macro."]
            impl #impl_generics #trait_name for #struct_name #ty_generics #where_clause {
                #render_window_methods

                #immutable_render
            }
        }
    } else {
        quote! {
            #[doc = "Renders [`Self`] in the immediate gui. The code was automatically generated using the derive macro."]
            impl #impl_generics #trait_name for #struct_name #ty_generics #where_clause {
                #render_window_methods

                #immutable_render

                #mutable_render
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proc_macro2::TokenStream;

    use crate::{
        attributes::AttributeHasDocumentation, derive_imgui_presentable_impl,
        derive_imgui_presentable_impl_for_backends,
    };

    use super::*;

    fn get_macro_from_statement(statement: &syn::Stmt) -> Option<&syn::Macro> {
        match statement {
            syn::Stmt::Macro(mac) => Some(&mac.mac),
            _ => None,
        }
    }

    fn get_self_type_from_impl(item_impl: &syn::ItemImpl) -> Option<String> {
        match item_impl.self_ty.as_ref() {
            syn::Type::Path(path) => Some(path.path.segments.last()?.ident.to_string()),
            _ => None,
        }
    }

    fn get_functions_from_impl(item_impl: &syn::ItemImpl) -> Vec<&syn::ImplItemFn> {
        item_impl
            .items
            .iter()
            .filter_map(|f| match f {
                syn::ImplItem::Fn(f) => Some(f),
                _ => None,
            })
            .collect()
    }

    fn get_immutable_code_from_impl(item_impl: &syn::ItemImpl) -> Option<&[syn::Stmt]> {
        get_functions_from_impl(item_impl).iter().find_map(|f| {
            if f.sig.ident == "render_component" {
                Some(f.block.stmts.as_slice())
            } else {
                None
            }
        })
    }

    fn get_mutable_code_from_impl(item_impl: &syn::ItemImpl) -> Option<&[syn::Stmt]> {
        get_functions_from_impl(item_impl).iter().find_map(|f| {
            if f.sig.ident == "render_component_mut" {
                Some(f.block.stmts.as_slice())
            } else {
                None
            }
        })
    }

    fn get_ident_from_pat(pat: &syn::Pat) -> Option<&syn::PatIdent> {
        match pat {
            syn::Pat::Ident(ident) => Some(ident),
            _ => None,
        }
    }

    fn get_let_statement(statement: &syn::Stmt) -> Option<(String, bool, Option<&syn::Expr>)> {
        let local = match statement {
            syn::Stmt::Local(local) => local,
            _ => return None,
        };

        let ident = get_ident_from_pat(&local.pat)?;
        let binding_name = ident.ident.to_string();
        let is_mutable = ident.mutability.is_some();

        if let Some(init) = local.init.as_ref() {
            Some((binding_name, is_mutable, Some(init.expr.as_ref())))
        } else {
            Some((binding_name, is_mutable, None))
        }
    }

    fn assert_is_let_binding_declaration(
        statement: &syn::Stmt,
        field_name: &str,
        is_mutable: bool,
        has_init: bool,
    ) {
        let let_binding = get_let_statement(statement).unwrap();
        assert_eq!(let_binding.0, field_name);
        assert_eq!(let_binding.1, is_mutable);
        assert_eq!(let_binding.2.is_some(), has_init);
    }

    fn get_trait_name_from_impl(item_impl: &syn::ItemImpl) -> Option<String> {
        Some(
            item_impl
                .trait_
                .as_ref()?
                .1
                .segments
                .last()?
                .ident
                .to_string(),
        )
    }

    fn assert_uses_imgui_control(statement: &syn::Stmt, mutably: bool, _is_primitive: bool) {
        let regex = regex::Regex::new(
            r"(.*)ui\s*\.\s*[checkbox|disabled|input_scalar|tree_node_config]\s*",
        )
        .unwrap();
        let s = &statement.to_token_stream().to_string();
        assert!(regex.is_match(s), "Isn't an imgui control usage: {s}");

        if !mutably {
            // TODO: ideally should check for passing the data to the
            // ImGui control object: either &self.data or &mut self.data.
            let regex = regex::Regex::new(r"render_component\s*\(").unwrap();
            assert!(regex.is_match(s), "Isn't using the field: {s}");
        }
    }

    fn assert_semicolon(statement: &syn::Stmt) {
        assert!(matches!(
            statement,
            syn::Stmt::Expr(syn::Expr::Verbatim(_), Some(syn::token::Semi { spans: _ })),
        ));
    }

    fn assert_has_proper_immutable_implementation(
        item_impl: &syn::ItemImpl,
        number_of_fields: usize,
    ) {
        let code = get_immutable_code_from_impl(item_impl).unwrap();

        assert_eq!(code.len(), number_of_fields * 3);
        (0..number_of_fields).for_each(|i| {
            let statement = &code[i];
            assert_is_let_binding_declaration(statement, "field_name", false, true);
            // let statement = &code[i + 1];
            // assert_is_let_binding_declaration(statement, "data", true, true);
            let statement = &code[i + 1];
            assert_uses_imgui_control(statement, false, true);
            let statement = &code[i + 2];
            assert_semicolon(statement);
        })
    }

    fn assert_has_proper_mutable_implementation(
        item_impl: &syn::ItemImpl,
        number_of_fields: usize,
    ) {
        let code = get_mutable_code_from_impl(item_impl).unwrap();

        assert_eq!(code.len(), number_of_fields * 3);
        (0..number_of_fields).for_each(|i| {
            let statement = &code[i];
            assert_is_let_binding_declaration(statement, "field_name", false, true);
            let statement = &code[i + 1];
            assert_uses_imgui_control(statement, true, true);
            let statement = &code[i + 2];
            assert_semicolon(statement);
        })
    }

    #[test]
    fn produces_error_for_an_empty_struct() {
        let s = r#"
        #[derive(ImguiPresentation)]
        struct A;
        "#;
        let generated = derive_imgui_presentable_impl(TokenStream::from_str(s).unwrap());
        let compile_error: syn::Macro = syn::parse2(generated).unwrap();
        assert_eq!(
            compile_error.path.to_token_stream().to_string(),
            "compile_error"
        );
    }

    #[test]
    fn syn_parses_macro() {
        let input = r#"
        compile_error ! (" #[imgui_presentation]: unknown attributes: abcd") ;
        "#;
        let _compile_error: syn::Stmt = syn::parse2(TokenStream::from_str(input).unwrap()).unwrap();
    }

    #[test]
    fn produces_error_for_an_unknown_attribute() {
        let inputs = [
            r#"
                #[derive(ImguiPresentation)]
                #[imgui_presentation(abcd)]
                struct A {
                    field: bool,
                }
            "#,
            r#"
                #[derive(ImguiPresentation)]
                struct A {
                    #[imgui_presentation(abcd)]
                    field: bool,
                }
            "#,
            r#"
                #[derive(ImguiPresentation)]
                struct A {
                    #[imgui_presentation(abcd, abcd, abcd = abcd)]
                    field: bool,
                }
            "#,
        ];
        for s in inputs {
            let generated = derive_imgui_presentable_impl(TokenStream::from_str(s).unwrap());
            let statement: syn::Stmt = syn::parse2(generated).unwrap();
            let compile_error = get_macro_from_statement(&statement).unwrap();
            assert_eq!(
                compile_error.path.to_token_stream().to_string(),
                "compile_error"
            );
            assert!(compile_error.tokens.to_string().contains("abcd"));
        }
    }

    #[test]
    fn generates_for_struct_with_one_primitive_field() {
        let inputs = [
            r#"
            #[derive(ImguiPresentation)]
            struct A {
                field: bool,
            }
            "#,
            r#"
            #[derive(ImguiPresentation)]
            struct A {
                field: i32,
            }
            "#,
        ];
        for s in inputs {
            let generated = derive_imgui_presentable_impl_for_backends(
                TokenStream::from_str(s).unwrap(),
                &[Backend::Imgui],
            );
            println!("{generated}");
            let item_impl: syn::ItemImpl = syn::parse2(generated.clone())
                .map_err(|e| {
                    println!("{generated}");
                    e
                })
                .unwrap();
            // eprintln!("item impl: {item_impl:#?}");
            let docs: String = item_impl
                .attrs
                .iter()
                .map(|a| a.get_documentation_string().unwrap())
                .collect::<Vec<String>>()
                .join("");
            assert_eq!(
                docs,
                "Renders [`Self`] in the immediate gui. The code was automatically generated using the derive macro."
            );
            // The last in trait_ path segments must be "ImguiPresentable".
            assert!(matches!(
                get_trait_name_from_impl(&item_impl).unwrap().as_str(),
                "ImguiPresentable"
            ));
            // The type the trait is implemented for is "A".
            assert_eq!(get_self_type_from_impl(&item_impl).unwrap(), "A");
            // Has both, the immutable and mutable implementations
            // and windows.
            assert_eq!(item_impl.items.len(), 4);
            assert_has_proper_immutable_implementation(&item_impl, 1);
            assert_has_proper_mutable_implementation(&item_impl, 1);
        }
    }

    // TODO: enable this test.
    // #[test]
    // fn generates_for_struct_with_many_different_fields() {
    //     let inputs = [
    //         r#"
    //         #[derive(ImguiPresentation)]
    //         struct A {
    //             field_bool: bool,
    //             field_string: String,
    //             field_option_string: Option<String>,
    //         }
    //         "#,
    //         r#"
    //         #[derive(ImguiPresentation)]
    //         struct A {
    //             fields: Vec<i32>,
    //             fields_strings: Vec<String>,
    //             fields_option_strings: Vec<Option<String>>,
    //         }
    //         "#,
    //         r#"
    //         #[derive(ImguiPresentation)]
    //         struct A {
    //             fields: HashMap<i32, String>,
    //             fields_strings: HashMap<String, HashMap<String, String>>,
    //             fields_option_strings: Vec<Option<String>>,
    //         }
    //         "#,
    //     ];
    //     for s in inputs {
    //         let generated = derive_imgui_presentable_impl(TokenStream::from_str(s).unwrap());
    //         let item_impl: syn::ItemImpl = syn::parse2(generated).unwrap();
    //         // eprintln!("item impl: {item_impl:#?}");
    //         let docs: String = item_impl
    //             .attrs
    //             .iter()
    //             .map(|a| a.get_documentation_string().unwrap())
    //             .collect::<Vec<String>>()
    //             .join("");
    //         assert_eq!(
    //             docs,
    //             " Renders [`#name`] using [`imgui_presentable::ImguiPresentable`] derive macro."
    //         );
    //         // The last in trait_ path segments must be "ImguiPresentable".
    //         assert_eq!(
    //             get_trait_name_from_impl(&item_impl).unwrap(),
    //             "ImguiPresentable"
    //         );
    //         // The type the trait is implemented for is "A".
    //         assert_eq!(get_self_type_from_impl(&item_impl).unwrap(), "A");
    //         // Has both, the immutable and mutable implementations.
    //         assert_eq!(item_impl.items.len(), 2);
    //         assert_has_proper_immutable_implementation(&item_impl, 3);
    //         assert_has_proper_mutable_implementation(&item_impl, 3);
    //     }
    // }
}

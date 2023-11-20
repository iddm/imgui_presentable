use proc_macro2::Span;
use quote::quote;
use syn::Ident;

#[allow(clippy::too_many_arguments)]
fn generate_ui_field_for_pod_enum(
    ui: &Ident,
    enum_name: &Ident,
    field_idents_and_values: &[(Ident, syn::Expr)],
    mutable: bool,
) -> proc_macro2::TokenStream {
    let field_idents: Vec<Ident> = field_idents_and_values
        .iter()
        .map(|v| v.0.clone())
        .collect();

    let order_to_idents: Vec<proc_macro2::TokenStream> = field_idents_and_values
        .iter()
        .enumerate()
        .map(|(i, (f, _))| {
            quote! {
                #i => Self::#f
            }
        })
        .collect();

    let idents_to_order: Vec<proc_macro2::TokenStream> = field_idents_and_values
        .iter()
        .enumerate()
        .map(|(i, (f, _))| {
            quote! {
                Self::#f => #i
            }
        })
        .collect();

    if mutable {
        quote! {
            let mut current_value = match self {
                #(#idents_to_order,)*
                _ => unreachable!("All the fields were checked."),
            };

            let values = [
                #(stringify!(#field_idents),)*
            ];

            let used = #ui.combo_simple_string(
                &format!("{}##{:p}", stringify!(#enum_name), std::ptr::addr_of!(self)),
                &mut current_value,
                &values,
            );

            if used {
                *self = match current_value {
                    #(#order_to_idents,)*
                    _ => unreachable!("All the fields were checked."),
                }
            }
        }
    } else {
        quote! {
            let mut current_value = match self {
                #(#idents_to_order,)*
                _ => unreachable!("All the fields were checked."),
            };

            let values = [
                #(stringify!(#field_idents),)*
            ];

            #ui.disabled(true, || {
                let _used = #ui.combo_simple_string(
                    &format!("{}##{:p}", stringify!(#enum_name), std::ptr::addr_of!(self)),
                    &mut current_value,
                    &values,
                );
            });

        }
    }
}

fn derive_for_pod_enum(
    derive_input: syn::DeriveInput,
    enumm: syn::DataEnum,
) -> proc_macro2::TokenStream {
    let name = derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    let variants: Vec<_> = enumm
        .variants
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let expr: syn::Expr = syn::parse2(quote! { #i }).unwrap();
            (
                v.ident.clone(),
                v.discriminant.clone().map(|d| d.1).unwrap_or(expr),
            )
        })
        .collect();
    let variants_count = variants.len();
    if variants_count == 0 {
        return quote! { compile_error!("Deriving ImguiPresentation for an empty enum is pointless.") };
    }

    let ui_ident = syn::Ident::new("ui", Span::call_site());
    let extent_ident = syn::Ident::new("extent", Span::call_site());

    let ui_elements = generate_ui_field_for_pod_enum(&ui_ident, &name, &variants, false);
    let ui_elements_mut = generate_ui_field_for_pod_enum(&ui_ident, &name, &variants, true);

    quote! {
        /// # Renders [`#name`] using
        /// [`imgui_presentable::ImguiPresentable`] derive macro.
        impl #impl_generics imgui_presentable::ImguiPresentable for #name #ty_generics #where_clause {
            fn render_component(&self, #ui_ident: &imgui::Ui, #extent_ident: imgui_presentable::Extent) {
                #ui_elements;
            }

            fn render_component_mut(&mut self, #ui_ident: &imgui::Ui, #extent_ident: imgui_presentable::Extent) {
                #ui_elements_mut;
            }
        }
    }
}

/// Derives the [`ImguiPresentable`] for an enum.
pub(crate) fn derive_for_enum(
    derive_input: syn::DeriveInput,
    enumm: syn::DataEnum,
) -> proc_macro2::TokenStream {
    let is_pod_enum = enumm.variants.iter().any(|v| v.fields.is_empty());

    if is_pod_enum {
        return derive_for_pod_enum(derive_input, enumm);
    }

    quote! { compile_error!("Only POD enums are supported as of this moment.") }
}

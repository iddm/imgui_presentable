use enum_impl::derive_for_enum;
use quote::quote;
use struct_impl::derive_for_struct;
use syn::Data;

mod attributes;
mod enum_impl;
mod struct_impl;

fn derive_imgui_presentable_impl(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let derive_input = {
        let tokens = tokens.clone();
        match syn::parse2::<syn::DeriveInput>(tokens) {
            Ok(data) => data,
            Err(err) => {
                return err.to_compile_error();
            }
        }
    };

    match derive_input.data.clone() {
        Data::Struct(strukt) => derive_for_struct(derive_input, strukt),
        Data::Enum(e) => derive_for_enum(derive_input, e),
        _ => quote! { compile_error!("Only structs and enums are supported.") },
    }
}

/// Generates the ImGui representation for a struct or an enum.
///
/// # Options
///
/// It is possible to configure the code generation with the use of the
/// [`#[imgui_presentation]`] procedural macro attribute, which has the
/// following options implemented:
///
/// - `readonly` makes a struct or a field have only immutable
/// presentation.
/// - `skip` skips the code generation for this field.
/// - `rename` renames a struct or a field in the generated
/// presentation code.
/// - `tooltip` changes the hint text for a field or a struct.
/// - `buttons` specifies the custom buttons to generate, can only be
/// specified on a struct/enum.
#[proc_macro_derive(ImguiPresentation, attributes(imgui_presentation))]
pub fn derive_imgui_presentable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_imgui_presentable_impl(tokens.into()).into()
}

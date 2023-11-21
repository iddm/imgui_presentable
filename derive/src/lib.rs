use enum_impl::derive_for_enum;
use quote::quote;
use struct_impl::derive_for_struct;
use syn::Data;

mod attributes;
mod enum_impl;
mod struct_impl;

#[cfg(all(feature = "imgui_backend", feature = "egui_backend"))]
compile_error!("Only one backend has to be chosen as a feature: either egui or imgui.");

#[cfg(not(any(feature = "imgui_backend", feature = "egui_backend")))]
compile_error!(
    "At least one backend has to be specified in the feature list: either egui or imgui."
);

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
/// - `format` (only for scalars) allows to set custom display format.
/// - `tooltip` changes the hint text for a field or a struct.
/// - `button` allows to generated custom buttons, can only be
/// specified on a struct/enum.
///
/// # Examples
///
/// ## Readonly
///
/// To make a whole struct read-only (so not render it allowing to
/// change the values) just use the `readonly` attribute:
///
/// ```rust
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// #[imgui_presentation(readonly)]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// To make a field read-only, it is allowed to specify the `readonly`
/// attribute on a field:
///
/// ```rust
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// pub struct A {
///     #[imgui_presentation(readonly)]
///     read_only_field: i32,
///     ordinary_field: i32,
/// }
/// ```
///
/// ## Skip
///
/// To skip generating a Imgui UI element for a field, use the `skip`
/// attribute:
///
/// ```rust
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// pub struct A {
///     #[imgui_presentation(skip)]
///     skipped_field: i32,
///     ordinary_field: i32,
/// }
/// ```
///
/// ## Rename
///
/// The code generator cannot make assumptions as to how it is the best
/// to name the user structure, and so just uses its full name. If a
/// more human-readable name is required, one can always specify the
/// name to appear in Imgui using the `rename` attribute:
///
/// ```rust
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// #[imgui_presentation(rename = "A great struct")]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// The same goes for the fields:
///
/// ```rust
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// #[imgui_presentation(rename = "A great struct")]
/// pub struct A {
///     #[imgui_presentation(rename = "A great field of a great struct")]
///     field: i32,
/// }
/// ```
///
/// ## Tooltip
///
/// The code generator grabs the doc-comment an enum, structure or a
/// field is annotated with and creates a tooltip for this field with
/// the text from that doc-comment. If the tooltip should be changed,
/// the `tooltip` attribute can be used for structs, enums and fields:
///
/// ```rust
/// // This is a struct with a tooltip from the doc-comment:
///
/// /// This is a great struct.
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// To change the tooltip text from `"This is a great struct"`, we use
/// the `tooltip` attribute as follows:
///
/// ```rust
/// #[imgui_presentation(tooltip = "This is not a great struct.")]
/// #[derive(Builder, Debug, Clone, ImguiPresentation)]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// The same goes for the enums, struct and enum fields.
///
/// ## Buttons
///
/// Buttons are specified once per attribute string, in the format of:
///
/// ```
/// button(<"button title"> : <"method name to call">)
/// ```
///
/// ```rust
/// #[derive(Builder, Debug, Clone, serde::Serialize, serde::Deserialize, ImguiPresentation)]
/// #[imgui_presentation(button("Hello world": "hello_world"))]
/// #[imgui_presentation(button("Bye world": "bye"))]
/// pub struct A {
/// }
///
/// impl A {
///     fn hello_world(&mut self) {
///         println!("The \"Hello world\" button was pressed.");
///     }
///
///     fn bye(&mut self) {
///         println!("The \"Bye world\" button was pressed.");
///     }
/// }
/// ```
///
/// ## Tuple structs
///
/// The tuple structs are supported exactly the same way as the ordinary
/// structs, except for the field names, which:
///
/// 1. In case there is only one field (`struct.0`), is prefixed with
/// the struct type.
/// 2. In case there are more fields, prefixed with their order numbers.
///
/// ## Enums
///
/// Only simple enums with simple variants are supported. Such enums
/// appear as a ComboBox which allows to select a new value from a set
/// of available variants of the enum, for example:
///
/// ```rust
/// /// The languages the engine supports.
/// #[derive(
///     Copy,
///     Default,
///     Clone,
///     Debug,
///     Eq,
///     PartialEq,
///     Ord,
///     PartialOrd,
///     Hash,
///     ImguiPresentation,
/// )]
/// pub enum Language {
///     /// English language.
///     #[default]
///     English,
///     /// Dutch language.
///     Dutch,
/// }
///
#[proc_macro_derive(ImguiPresentation, attributes(imgui_presentation))]
pub fn derive_imgui_presentable(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_imgui_presentable_impl(tokens.into()).into()
}

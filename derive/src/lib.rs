use std::str::FromStr;

use enum_impl::derive_for_enum;
use quote::quote;
use struct_impl::derive_for_struct;
use syn::Data;

mod attributes;
mod enum_impl;
mod struct_impl;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum Backend {
    #[allow(dead_code)]
    Imgui,
    #[allow(dead_code)]
    Egui,
}

impl FromStr for Backend {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_ref() {
            "imgui" => Self::Imgui,
            "egui" => Self::Egui,
            _ => return Err(format!("{s} is an unknown backend.")),
        })
    }
}

#[cfg(not(any(feature = "imgui_backend", feature = "egui_backend")))]
compile_error!(
    "At least one backend has to be specified in the feature list: either egui or imgui. The derive macro is useless otherwise."
);

fn derive_imgui_presentable_impl_for_backends(
    tokens: proc_macro2::TokenStream,
    backends: &[Backend],
) -> proc_macro2::TokenStream {
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
        Data::Struct(strukt) => derive_for_struct(derive_input, strukt, backends),
        Data::Enum(enumm) => derive_for_enum(derive_input, enumm, backends),
        _ => quote! { compile_error!("Only structs and enums are supported.") },
    }
}

fn derive_imgui_presentable_impl(tokens: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let backends = [
        #[cfg(feature = "imgui_backend")]
        Backend::Imgui,
        #[cfg(feature = "egui_backend")]
        Backend::Egui,
    ];

    derive_imgui_presentable_impl_for_backends(tokens, &backends)
}

/// Generates the immediate gui (ImGui or egui) representation for a
/// struct or an enum.
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
/// - `speed` (only for scalars) allows to set custom speed of
/// - `range` (only for scalars) allows to set a range of values the
/// scalar object can have.
/// the value change when dragging.
/// - `tooltip` changes the hint text for a field or a struct.
/// - `button` allows to generated custom buttons, can only be
/// specified on a struct/enum.
/// - `backend` allows a struct or enum to specify the backend it needs.
/// only the chosen backend code will be derived.
/// - `main_menu_item` allows to specify a main menu item. The main
/// menu items are always visible and start at the top-left corner of
/// a window.
///
/// # Examples
///
/// ## Readonly
///
/// To make a whole struct read-only (so not render it allowing to
/// change the values) just use the `readonly` attribute:
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// #[imgui_presentation(readonly)]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// To make a field read-only, it is allowed to specify the `readonly`
/// attribute on a field:
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
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
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
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
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// #[imgui_presentation(rename = "A great struct")]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// The same goes for the fields:
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// #[imgui_presentation(rename = "A great struct")]
/// pub struct A {
///     #[imgui_presentation(rename = "A great field of a great struct")]
///     field: i32,
/// }
/// ```
///
/// ## Display format
///
/// For the primitive Rust types it is possible to override the default
/// display formatter using a custom formatter in the "printf" format.
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// pub struct A {
///     #[imgui_presentation(format = "%.2f")]
///     value: f32,
/// }
/// ```
///
/// ## Speed
///
/// For the primitive Rust types it is possible to override the default
/// change speed, by supplying an [`f32`] number:
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// pub struct A {
///     #[imgui_presentation(speed = 0.001f32)]
///     value: f32,
/// }
/// ```
///
/// ## Range of values
///
/// For the primitive Rust types, it is possible to specify the range
/// of values which is possible to set via GUI.
///
/// ```
/// #[derive(ImguiPresentation)]
/// pub struct A {
///     // Just use the basic Rust range syntax here.
///     #[imgui_presentation(range = 0.0f32..0.1f32)]
///     value: f32,
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
/// ```rust,ignore
/// // This is a struct with a tooltip from the doc-comment:
///
/// /// This is a great struct.
/// #[derive(ImguiPresentation)]
/// pub struct A {
///     field: i32,
/// }
/// ```
///
/// To change the tooltip text from `"This is a great struct"`, we use
/// the `tooltip` attribute as follows:
///
/// ```rust,ignore
/// #[imgui_presentation(tooltip = "This is not a great struct.")]
/// #[derive(ImguiPresentation)]
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
/// ```ignore,no_run
/// button(<"button title"> : <"method name to call">)
/// ```
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
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
/// ## Main menu bar items
///
/// Buttons are specified once per attribute string, in the format of:
///
/// ```ignore,no_run
/// main_menu_item(<"title"> : <"method name to call">)
/// ```
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// #[imgui_presentation(main_menu_item("File": "file_menu_clicked"))]
/// #[imgui_presentation(main_menu_item("About": "about"))]
/// pub struct A {
/// }
///
/// impl A {
///     fn file_menu_clicked(&mut self) {
///         println!("The \"File\" main menu item was clicked.");
///     }
///
///     fn about(&mut self) {
///         println!("The \"About\" main menu item was clicked.");
///     }
/// }
/// ```
///
/// ## Backends
///
/// To specify a backend for the code generation for a struct, use the
/// `backend` attribute:
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// #[imgui_presentation(backend = "imgui")]
/// pub struct A {
/// }
/// ```
///
/// ## Backends
///
/// To specify a backend for the code generation for a struct, use the
/// `backend` attribute:
///
/// ```rust,ignore
/// #[derive(ImguiPresentation)]
/// #[imgui_presentation(backend = "imgui")]
/// pub struct A {
/// }
/// ```
///
/// Usually, the backend specification isn't required, as only
/// one of the backends is built due to only one being selected as a
/// crate feature. However, by default, both are generated as both
/// features are the default ones, and if the default features aren't
/// disabled for the crate, the compiler will always try to generate the
/// code for both the backends even if only one is actually being used.
/// For that reason, to direct the derive macro to generate the code
/// for a struct or an enum for only one backend out of others which are
/// also available, the `backend` attribute should be used. If it is
/// known in advance that only one backend will be used, just disable
/// the default features for this crate and specify the ones you need.
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
/// ```rust,ignore
/// /// The languages the engine supports.
/// #[derive(
///     Default,
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

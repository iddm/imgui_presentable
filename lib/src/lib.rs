//! Imgui Presentable.
//!
//! Here are the main interfaces which allow the users to render enums
//! or structs in ImGui or egui effortlessly.
//!
//! To switch between the gui backends (ImGui and egui), use the
//! corresponding crate features. Both the features can be enabled
//! simultaneously, if necessary.
//!
//! The crate already provides some implementations for the basic Rust
//! types and some other types from other crates, feature-gated.
//!
//! For the collections, the trait is implemented automatically if the
//! element type of the collection implements the [`ImguiPresentable`]
//! or the [`EguiPresentable`] trait itself.

pub use imgui_presentable_derive::*;
mod basic_types;
#[cfg(feature = "glam")]
mod glam_types;
mod std_types;

/// The width and height of the viewport used by ImGUI.
#[cfg(feature = "imgui_backend")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Extent {
    /// The width of the viewport (in pixels).
    pub width: u16,
    /// The height of the viewport (in pixels).
    pub height: u16,
}
/// Allows the implementing object to be rendered as an ImGUI component.
#[cfg(feature = "imgui_backend")]
pub trait ImguiPresentable {
    /// Renders the implementor as a stand-alone window not allowing to
    /// change the values.
    ///
    /// # Note
    ///
    /// This is the default implementation. The code may be overriden
    /// and changed to some other window creation code, depending on
    /// the code generation options used.
    fn render_window(&self, ui: &imgui::Ui, extent: Extent) {
        ui.window(std::any::type_name::<Self>())
            .resizable(true)
            .collapsible(true)
            .bg_alpha(0.7f32)
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .menu_bar(false)
            .build(|| self.render_component(ui, extent));
    }

    /// Renders the implementor as a sub-component not allowing to
    /// change the values.
    fn render_component(&self, ui: &imgui::Ui, extent: Extent);

    /// Renders the implementor as a stand-alone window allowing to
    /// change the values.
    ///
    /// # Note
    ///
    /// This is the default implementation. The code may be overriden
    /// and changed to some other window creation code, depending on
    /// the code generation options used.
    fn render_window_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        ui.window(std::any::type_name::<Self>())
            .resizable(true)
            .collapsible(true)
            .bg_alpha(0.7f32)
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .menu_bar(false)
            .build(|| self.render_component_mut(ui, extent));
    }

    /// Renders the implementor as a sub-component allowing to change
    /// the values.
    ///
    /// # Note
    ///
    /// If not re-implemented, the default implementation shows the
    /// immutable UI.
    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        // ui.text("This struct doesn't provide a mutable ui.");
        self.render_component(ui, extent);
    }
}

/// Allows the implementing object to be rendered as an eGUI component.
#[cfg(feature = "egui_backend")]
pub trait EguiPresentable {
    /// Renders the implementor as a stand-alone window not allowing to
    /// change the values.
    fn render_window(&self, context: &egui::Context) {
        egui::Window::new(std::any::type_name::<Self>())
            .show(context, |ui| self.render_component(ui));
    }

    /// Renders the implementor as a sub-component not allowing to
    /// change the values.
    fn render_component(&self, ui: &mut egui::Ui);

    /// Renders the implementor as a stand-alone window allowing to
    /// change the values.
    fn render_window_mut(&mut self, context: &egui::Context) {
        egui::Window::new(std::any::type_name::<Self>())
            .show(context, |ui| self.render_component_mut(ui));
    }

    /// Renders the implementor as a sub-component allowing to change
    /// the values.
    ///
    /// # Note
    ///
    /// If not re-implemented, the default implementation shows the
    /// immutable UI.
    fn render_component_mut(&mut self, ui: &mut egui::Ui) {
        self.render_component(ui);
    }
}

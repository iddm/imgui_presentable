# Immediate Gui Presentable

[![CI](https://github.com/iddm/imgui_presentable/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/imgui_presentable/actions/workflows/ci.yml)
[![Crates](https://img.shields.io/crates/v/imgui_presentable.svg)](https://crates.io/crates/imgui_presentable)
[![Docs](https://docs.rs/imgui_presentable/badge.svg)](https://docs.rs/imgui_presentable)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A derive-macro for easily showing your structs as a GUI component using
[egui](https://github.com/emilk/egui) or [imgui-rs](https://github.com/imgui-rs/imgui-rs).

The name of this crate (imgui_presentable) may seem confusing and being
only limited to ImGui. Here, one may think of the name in the broad
sense instead - "Immediate Gui", so that it is not limited to ImGui and
never should be. The goal of this crate is to implement all the backends
possible, and preferrably hide all the implementation detail behind the
derive-macro.

## Example (using ImGui)

```rust
/// Describes a game scene.
#[derive(Builder, Debug, Clone, serde::Serialize, serde::Deserialize, ImguiPresentation)]
#[imgui_presentation(button("Reload from file": "reload"))]
#[imgui_presentation(button("Render": "reload_imgui"))]
pub struct Scene {
    #[builder(default)]
    #[imgui_presentation(skip)]
    magic_header: MagicHeader,
    #[imgui_presentation(skip)]
    id: uuid::Uuid,
    /// Scene path.
    #[imgui_presentation(skip)]
    path: std::path::PathBuf,
    /// Objects within this scene.
    data: Data,
    #[builder(default)]
    physics_components: IndexedComponentsStorage<PhysicsComponent>,
    #[builder(default)]
    mesh_components: IndexedComponentsStorage<MeshComponent>,
    #[builder(default)]
    entities: HashSet<Entity>,
    #[serde(skip)]
    #[builder(default)]
    entity_change_set: EntityChangeSet,
    #[builder(default)]
    farthest_object_distance: f32,
}
```
Deriving the `ImguiPresentation` implements the following trait, namely overrides the `render_component` and `render_component_mut` where possible:

```rust
/// Allows the implementing object to be rendered as an ImGUI component.
pub trait ImguiPresentable {
    /// Renders the implementor as a stand-alone window not allowing to
    /// change the values.
    fn render_window(&self, ui: &imgui::Ui, extent: Extent) {
        ui.window(std::any::type_name::<Self>())
            .resizable(true)
            .collapsible(true)
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
            .build(|| self.render_component(ui, extent));
    }

    /// Renders the implementor as a sub-component not allowing to
    /// change the values.
    fn render_component(&self, ui: &imgui::Ui, extent: Extent);

    /// Renders the implementor as a stand-alone window allowing to
    /// change the values.
    fn render_window_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        ui.window(std::any::type_name::<Self>())
            .resizable(true)
            .collapsible(true)
            .position([0.0, 0.0], imgui::Condition::FirstUseEver)
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
```

Now, whenever it is needed to render the struct using ImGui, in the
render loop, when a context to ImGui is obtained and a new frame drawing
has begun, simply:

```rust
// Before the loop we create the object we want to render:
let mut scene = Scene::new();
// And initialise imgui:
let imgui = _; // obtain the imgui object
// And store the extent of the window:
let extent = imgui_presentable::Extent {
    width: window.width,
    height: window.height,
};

// Now, in the render loop:
let context = imgui.get_context();
let ui = context.new_frame();

// Render the component in a separate window:
scene.render_window_mut(ui, extent);

// Then finish the ImGui frame:
let draw_data = imgui.render();
// And render it using what you can.
```

This leads to this:

![Image](screenshot.png "The Scene struct is shown in ImGui")

## Backends

The supported backends:

- egui
- ImGui

The backend is selectable via the crate features.

For each of the backends, a similar but separate trait exists. Though,
the same trait could have just been changed, it is much more practical
and agile to allow both features to co-exist and this cannot be done
using a single trait, unfortunately.

## License

MIT

//! This module implements the [`ImguiPresentable`] for the basic Rust
//! types, which are not collections.

// fn label_with_address<T>(label: &str, t: &T) -> String {
//     format!("{label}##{:p}", std::ptr::addr_of!(t))
// }

use crate::{Extent, ImguiPresentable};

macro_rules! define_for_scalar {
    ($scalar_type: ty) => {
        impl ImguiPresentable for $scalar_type {
            fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
                let type_name = std::any::type_name::<$scalar_type>();
                let mut data = *self;
                ui.disabled(true, || {
                    let _ = ui
                        .input_scalar(&format!("{type_name}##{self:p}"), &mut data)
                        .read_only(true)
                        .build();
                });
            }

            fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
                let type_name = std::any::type_name::<$scalar_type>();
                let _ = ui
                    .input_scalar(&format!("{type_name}##{self:p}"), self)
                    .build();
            }
        }
    };
}

define_for_scalar!(i8);
define_for_scalar!(u8);
define_for_scalar!(i16);
define_for_scalar!(u16);
define_for_scalar!(i32);
define_for_scalar!(u32);
define_for_scalar!(i64);
define_for_scalar!(u64);
define_for_scalar!(f32);
define_for_scalar!(f64);
define_for_scalar!(isize);
define_for_scalar!(usize);

fn bool_to_string(value: bool) -> &'static str {
    const YES: &str = "yes";
    const NO: &str = "no";

    if value {
        YES
    } else {
        NO
    }
}

impl ImguiPresentable for bool {
    fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
        let mut data = *self;
        let text = bool_to_string(*self);
        ui.disabled(true, || {
            let _ = ui.checkbox(format!("{text}##{:p}", std::ptr::addr_of!(self)), &mut data);
        });
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
        let text = bool_to_string(*self);
        let _ = ui.checkbox(&format!("{text}##{self:p}"), self);
    }
}

impl ImguiPresentable for String {
    fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
        ui.text(self);
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
        let _ = ui.input_text(&format!("{self}##{self:p}"), self).build();
    }
}

impl ImguiPresentable for &str {
    fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
        ui.text(self);
    }
}

// impl ImguiPresentable for &String {
//     fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
//         ui.text(format!("{}##{:p}", self, std::ptr::addr_of!(self)));
//     }
// }

// impl ImguiPresentable for &mut String {
//     fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
//         ui.text(format!("{}##{:p}", self, std::ptr::addr_of!(self)));
//     }

//     fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
//         let _ = ui
//             .input_text(&format!("##{:p}", std::ptr::addr_of!(self)), self)
//             .build();
//     }
// }

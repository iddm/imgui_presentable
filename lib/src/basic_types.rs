//! This module implements the [`ImguiPresentable`] for the basic Rust
//! types, which are not collections.

// fn label_with_address<T>(label: &str, t: &T) -> String {
//     format!("{label}##{:p}", std::ptr::addr_of!(t))
// }

#[cfg(feature = "imgui_backend")]
mod imgui_backend {
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
}
#[cfg(feature = "imgui_backend")]
pub use imgui_backend::*;

#[cfg(feature = "egui_backend")]
mod egui_backend {
    use crate::ImguiPresentable;

    macro_rules! define_for_scalar {
        ($scalar_type: ty) => {
            impl ImguiPresentable for $scalar_type {
                fn render_component(&self, ui: &mut egui::Ui) {
                    let type_name = std::any::type_name::<$scalar_type>();
                    let mut data = *self;
                    ui.add_enabled_ui(false, |ui: &mut egui::Ui| {
                        let _ = ui.add(egui::DragValue::new(&mut data).prefix(type_name));
                    });
                }

                fn render_component_mut(&mut self, ui: &mut egui::Ui) {
                    let type_name = std::any::type_name::<$scalar_type>();
                    let mut data = *self;
                    let _ = ui.add(egui::DragValue::new(&mut data).prefix(type_name));
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

    // Thanks for the code to the egui project:
    // <https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/toggle_switch.rs>
    fn bool_switch_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }
        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }

    impl ImguiPresentable for bool {
        fn render_component(&self, ui: &mut egui::Ui) {
            let mut data = *self;
            ui.add_enabled_ui(false, |ui: &mut egui::Ui| {
                let _ = bool_switch_ui(ui, &mut data);
            });
        }

        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let _ = bool_switch_ui(ui, self);
        }
    }

    impl ImguiPresentable for String {
        fn render_component(&self, ui: &mut egui::Ui) {
            ui.label(self);
        }

        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let _ = ui.text_edit_singleline(self);
        }
    }

    impl ImguiPresentable for &str {
        fn render_component(&self, ui: &mut egui::Ui) {
            ui.label(*self);
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
}
#[cfg(feature = "egui_backend")]
pub use egui_backend::*;

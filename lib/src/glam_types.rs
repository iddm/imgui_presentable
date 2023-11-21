//! Provides the implementations of the [ImguiPresentable] trait for the
//! [`glam`] crate types.

#[cfg(feature = "imgui_backend")]
mod imgui_backend {
    use crate::{Extent, ImguiPresentable};

    impl ImguiPresentable for glam::Vec4 {
        fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x"),
                    imgui::TableColumnSetup::new("y"),
                    imgui::TableColumnSetup::new("z"),
                    imgui::TableColumnSetup::new("w"),
                ],
            ) {
                ui.table_next_column();
                self.x.render_component_mut(ui, extent);

                ui.table_next_column();
                self.y.render_component_mut(ui, extent);

                ui.table_next_column();
                self.z.render_component_mut(ui, extent);

                ui.table_next_column();
                self.w.render_component_mut(ui, extent);

                table.end();
            }
        }

        fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x"),
                    imgui::TableColumnSetup::new("y"),
                    imgui::TableColumnSetup::new("z"),
                    imgui::TableColumnSetup::new("w"),
                ],
            ) {
                ui.table_next_column();
                self.x.render_component(ui, extent);

                ui.table_next_column();
                self.y.render_component(ui, extent);

                ui.table_next_column();
                self.z.render_component(ui, extent);

                ui.table_next_column();
                self.w.render_component(ui, extent);

                table.end();
            }
        }
    }

    impl ImguiPresentable for glam::Vec3 {
        fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x"),
                    imgui::TableColumnSetup::new("y"),
                    imgui::TableColumnSetup::new("z"),
                ],
            ) {
                ui.table_next_column();
                self.x.render_component_mut(ui, extent);

                ui.table_next_column();
                self.y.render_component_mut(ui, extent);

                ui.table_next_column();
                self.z.render_component_mut(ui, extent);

                table.end();
            }
        }

        fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x"),
                    imgui::TableColumnSetup::new("y"),
                    imgui::TableColumnSetup::new("z"),
                ],
            ) {
                ui.table_next_column();
                self.x.render_component(ui, extent);

                ui.table_next_column();
                self.y.render_component(ui, extent);

                ui.table_next_column();
                self.z.render_component(ui, extent);

                table.end();
            }
        }
    }

    impl ImguiPresentable for glam::Vec2 {
        fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x"),
                    imgui::TableColumnSetup::new("y"),
                ],
            ) {
                ui.table_next_column();
                self.x.render_component_mut(ui, extent);

                ui.table_next_column();
                self.y.render_component_mut(ui, extent);

                table.end();
            }
        }

        fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x"),
                    imgui::TableColumnSetup::new("y"),
                ],
            ) {
                ui.table_next_column();
                self.x.render_component(ui, extent);

                ui.table_next_column();
                self.y.render_component(ui, extent);

                table.end();
            }
        }
    }

    fn create_vec4_for_mat4(ui: &imgui::Ui, vec: &glam::Vec4, prefix: &str) {
        let _ = {
            let mut value = vec.x;
            ui.input_float(
                &format!("{prefix}_0##{:p}", std::ptr::addr_of!(vec.x)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
        let _ = {
            let mut value = vec.y;
            ui.input_float(
                &format!("{prefix}_1##{:p}", std::ptr::addr_of!(vec.y)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
        let _ = {
            let mut value = vec.z;
            ui.input_float(
                &format!("{prefix}_2##{:p}", std::ptr::addr_of!(vec.z)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
        let _ = {
            let mut value = vec.w;
            ui.input_float(
                &format!("{prefix}_3##{:p}", std::ptr::addr_of!(vec.w)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
    }

    fn create_vec4_for_mat4_mut(ui: &imgui::Ui, vec: &mut glam::Vec4, prefix: &str) {
        let _ = {
            ui.input_float(
                &format!("{prefix}_0##{:p}", std::ptr::addr_of!(vec.x)),
                &mut vec.x,
            )
            .build()
        };
        let _ = {
            ui.input_float(
                &format!("{prefix}_1##{:p}", std::ptr::addr_of!(vec.y)),
                &mut vec.y,
            )
            .build()
        };
        let _ = {
            ui.input_float(
                &format!("{prefix}_2##{:p}", std::ptr::addr_of!(vec.z)),
                &mut vec.z,
            )
            .build()
        };
        let _ = {
            ui.input_float(
                &format!("{prefix}_3##{:p}", std::ptr::addr_of!(vec.w)),
                &mut vec.w,
            )
            .build()
        };
    }

    fn create_vec3_for_mat3(ui: &imgui::Ui, vec: &glam::Vec3, prefix: &str) {
        let _ = {
            let mut value = vec.x;
            ui.input_float(
                &format!("{prefix}_0##{:p}", std::ptr::addr_of!(vec.x)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
        let _ = {
            let mut value = vec.y;
            ui.input_float(
                &format!("{prefix}_1##{:p}", std::ptr::addr_of!(vec.y)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
        let _ = {
            let mut value = vec.z;
            ui.input_float(
                &format!("{prefix}_2##{:p}", std::ptr::addr_of!(vec.z)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
    }

    fn create_vec3_for_mat3_mut(ui: &imgui::Ui, vec: &mut glam::Vec3, prefix: &str) {
        let _ = {
            ui.input_float(
                &format!("{prefix}_0##{:p}", std::ptr::addr_of!(vec.x)),
                &mut vec.x,
            )
            .build()
        };
        let _ = {
            ui.input_float(
                &format!("{prefix}_1##{:p}", std::ptr::addr_of!(vec.x)),
                &mut vec.y,
            )
            .build()
        };
        let _ = {
            ui.input_float(
                &format!("{prefix}_2##{:p}", std::ptr::addr_of!(vec.x)),
                &mut vec.z,
            )
            .build()
        };
    }

    fn create_vec2_for_mat2(ui: &imgui::Ui, vec: &glam::Vec2, prefix: &str) {
        let _ = {
            let mut value = vec.x;
            ui.input_float(
                &format!("{prefix}_0##{:p}", std::ptr::addr_of!(vec.x)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
        let _ = {
            let mut value = vec.y;
            ui.input_float(
                &format!("{prefix}_1##{:p}", std::ptr::addr_of!(vec.x)),
                &mut value,
            )
            .read_only(true)
            .build()
        };
    }

    fn create_vec2_for_mat2_mut(ui: &imgui::Ui, vec: &mut glam::Vec2, prefix: &str) {
        let _ = {
            ui.input_float(
                &format!("{prefix}_0##{:p}", std::ptr::addr_of!(vec.x)),
                &mut vec.x,
            )
            .build()
        };
        let _ = {
            ui.input_float(
                &format!("{prefix}_1##{:p}", std::ptr::addr_of!(vec.x)),
                &mut vec.y,
            )
            .build()
        };
    }

    impl ImguiPresentable for glam::Mat4 {
        fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x_axis"),
                    imgui::TableColumnSetup::new("y_axis"),
                    imgui::TableColumnSetup::new("z_axis"),
                    imgui::TableColumnSetup::new("w_axis"),
                ],
            ) {
                ui.table_next_column();
                create_vec4_for_mat4(ui, &self.x_axis, "x");

                ui.table_next_column();
                create_vec4_for_mat4(ui, &self.y_axis, "y");

                ui.table_next_column();
                create_vec4_for_mat4(ui, &self.z_axis, "z");

                ui.table_next_column();
                create_vec4_for_mat4(ui, &self.w_axis, "w");

                table.end();
            }

            if ui.button("Clipboard") {
                ui.set_clipboard_text(format!("{self:#?}"));
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Copies the debug representation to clipboard.");
            }
        }

        fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x_axis"),
                    imgui::TableColumnSetup::new("y_axis"),
                    imgui::TableColumnSetup::new("z_axis"),
                    imgui::TableColumnSetup::new("w_axis"),
                ],
            ) {
                ui.table_next_column();
                create_vec4_for_mat4_mut(ui, &mut self.x_axis, "x");

                ui.table_next_column();
                create_vec4_for_mat4_mut(ui, &mut self.y_axis, "y");

                ui.table_next_column();
                create_vec4_for_mat4_mut(ui, &mut self.z_axis, "z");

                ui.table_next_column();
                create_vec4_for_mat4_mut(ui, &mut self.w_axis, "w");

                table.end();
            }

            if ui.button(format!("Identity##{self:p}")) {
                *self = Self::IDENTITY;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix to an identity matrix.");
            }

            ui.same_line();

            if ui.button(format!("Zero##{self:p}")) {
                *self = Self::ZERO;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix to an zero matrix.");
            }

            ui.same_line();

            if ui.button(format!("NaN##{self:p}")) {
                *self = Self::NAN;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix elements to f32::NaN.");
            }

            ui.same_line();

            if ui.button(format!("Clipboard##{self:p}")) {
                ui.set_clipboard_text(format!("{self:#?}"));
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Copies the debug representation to clipboard.");
            }
        }
    }

    impl ImguiPresentable for glam::Mat3 {
        fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x_axis"),
                    imgui::TableColumnSetup::new("y_axis"),
                    imgui::TableColumnSetup::new("z_axis"),
                ],
            ) {
                ui.table_next_column();
                create_vec3_for_mat3(ui, &self.x_axis, "x");

                ui.table_next_column();
                create_vec3_for_mat3(ui, &self.y_axis, "y");

                ui.table_next_column();
                create_vec3_for_mat3(ui, &self.z_axis, "z");

                table.end();
            }

            if ui.button(format!("Clipboard##{self:p}")) {
                ui.set_clipboard_text(format!("{self:#?}"));
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Copies the debug representation to clipboard.");
            }
        }

        fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x_axis"),
                    imgui::TableColumnSetup::new("y_axis"),
                    imgui::TableColumnSetup::new("z_axis"),
                ],
            ) {
                ui.table_next_column();
                create_vec3_for_mat3_mut(ui, &mut self.x_axis, "x");

                ui.table_next_column();
                create_vec3_for_mat3_mut(ui, &mut self.y_axis, "y");

                ui.table_next_column();
                create_vec3_for_mat3_mut(ui, &mut self.z_axis, "z");

                table.end();
            }

            if ui.button(format!("Identity##{self:p}")) {
                *self = Self::IDENTITY;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix to an identity matrix.");
            }

            ui.same_line();

            if ui.button(format!("Zero##{self:p}")) {
                *self = Self::ZERO;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix to an zero matrix.");
            }

            ui.same_line();

            if ui.button(format!("NaN##{self:p}")) {
                *self = Self::NAN;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix elements to f32::NaN.");
            }

            ui.same_line();

            if ui.button(format!("Clipboard##{self:p}")) {
                ui.set_clipboard_text(format!("{self:#?}"));
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Copies the debug representation to clipboard.");
            }
        }
    }

    impl ImguiPresentable for glam::Mat2 {
        fn render_component(&self, ui: &imgui::Ui, _extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x_axis"),
                    imgui::TableColumnSetup::new("y_axis"),
                ],
            ) {
                ui.table_next_column();
                create_vec2_for_mat2(ui, &self.x_axis, "x");

                ui.table_next_column();
                create_vec2_for_mat2(ui, &self.y_axis, "y");

                table.end();
            }

            if ui.button(format!("Clipboard##{self:p}")) {
                ui.set_clipboard_text(format!("{self:#?}"));
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Copies the debug representation to clipboard.");
            }
        }

        fn render_component_mut(&mut self, ui: &imgui::Ui, _extent: Extent) {
            let _id = ui.push_id(&format!("##{self:p}"));

            if let Some(table) = ui.begin_table_header(
                "values",
                [
                    imgui::TableColumnSetup::new("x_axis"),
                    imgui::TableColumnSetup::new("y_axis"),
                ],
            ) {
                ui.table_next_column();
                create_vec2_for_mat2_mut(ui, &mut self.x_axis, "x");

                ui.table_next_column();
                create_vec2_for_mat2_mut(ui, &mut self.y_axis, "y");

                table.end();
            }

            if ui.button(format!("Identity##{self:p}")) {
                *self = Self::IDENTITY;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix to an identity matrix.");
            }

            ui.same_line();

            if ui.button(format!("Zero##{self:p}")) {
                *self = Self::ZERO;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix to an zero matrix.");
            }

            ui.same_line();

            if ui.button(format!("NaN##{self:p}")) {
                *self = Self::NAN;
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Reset the matrix elements to f32::NaN.");
            }

            ui.same_line();

            if ui.button(format!("Clipboard##{self:p}")) {
                ui.set_clipboard_text(format!("{self:#?}"));
            }

            if ui.is_item_hovered() {
                ui.tooltip_text("Copies the debug representation to clipboard.");
            }
        }
    }
}

#[cfg(feature = "imgui_backend")]
pub use imgui_backend::*;

#[cfg(feature = "egui_backend")]
mod egui_backend {
    use crate::ImguiPresentable;
    use egui_extras::{Column, TableBuilder};

    impl ImguiPresentable for glam::Vec4 {
        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 4);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x");
                    });
                    header.col(|ui| {
                        ui.strong("y");
                    });
                    header.col(|ui| {
                        ui.strong("z");
                    });
                    header.col(|ui| {
                        ui.strong("w");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| self.x.render_component_mut(ui));
                        row.col(|ui| self.y.render_component_mut(ui));
                        row.col(|ui| self.z.render_component_mut(ui));
                        row.col(|ui| self.w.render_component_mut(ui));
                    });
                });
        }

        fn render_component(&self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 4);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x");
                    });
                    header.col(|ui| {
                        ui.strong("y");
                    });
                    header.col(|ui| {
                        ui.strong("z");
                    });
                    header.col(|ui| {
                        ui.strong("w");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| self.x.render_component(ui));
                        row.col(|ui| self.y.render_component(ui));
                        row.col(|ui| self.z.render_component(ui));
                        row.col(|ui| self.w.render_component(ui));
                    });
                });
        }
    }

    impl ImguiPresentable for glam::Vec3 {
        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 3);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x");
                    });
                    header.col(|ui| {
                        ui.strong("y");
                    });
                    header.col(|ui| {
                        ui.strong("z");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| self.x.render_component_mut(ui));
                        row.col(|ui| self.y.render_component_mut(ui));
                        row.col(|ui| self.z.render_component_mut(ui));
                    });
                });
        }

        fn render_component(&self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 3);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x");
                    });
                    header.col(|ui| {
                        ui.strong("y");
                    });
                    header.col(|ui| {
                        ui.strong("z");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| self.x.render_component(ui));
                        row.col(|ui| self.y.render_component(ui));
                        row.col(|ui| self.z.render_component(ui));
                    });
                });
        }
    }

    impl ImguiPresentable for glam::Vec2 {
        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 2);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x");
                    });
                    header.col(|ui| {
                        ui.strong("y");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| self.x.render_component_mut(ui));
                        row.col(|ui| self.y.render_component_mut(ui));
                    });
                });
        }

        fn render_component(&self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 2);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x");
                    });
                    header.col(|ui| {
                        ui.strong("y");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| self.x.render_component(ui));
                        row.col(|ui| self.y.render_component(ui));
                    });
                });
        }
    }

    fn create_vec4_for_mat4(ui: &mut egui::Ui, vec: &glam::Vec4, prefix: &str) {
        ui.add_enabled_ui(false, |ui| {
            let mut value = vec.x;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_0")));
            let mut value = vec.y;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_1")));
            let mut value = vec.z;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_2")));
            let mut value = vec.w;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_3")));
        });
    }

    fn create_vec4_for_mat4_mut(ui: &mut egui::Ui, vec: &mut glam::Vec4, prefix: &str) {
        let _ = ui.add(egui::DragValue::new(&mut vec.x).prefix(format!("{prefix}_0")));
        let _ = ui.add(egui::DragValue::new(&mut vec.y).prefix(format!("{prefix}_1")));
        let _ = ui.add(egui::DragValue::new(&mut vec.z).prefix(format!("{prefix}_2")));
        let _ = ui.add(egui::DragValue::new(&mut vec.w).prefix(format!("{prefix}_3")));
    }

    fn create_vec3_for_mat3(ui: &mut egui::Ui, vec: &glam::Vec3, prefix: &str) {
        ui.add_enabled_ui(false, |ui| {
            let mut value = vec.x;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_0")));
            let mut value = vec.y;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_1")));
            let mut value = vec.z;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_2")));
        });
    }

    fn create_vec3_for_mat3_mut(ui: &mut egui::Ui, vec: &mut glam::Vec3, prefix: &str) {
        let _ = ui.add(egui::DragValue::new(&mut vec.x).prefix(format!("{prefix}_0")));
        let _ = ui.add(egui::DragValue::new(&mut vec.y).prefix(format!("{prefix}_1")));
        let _ = ui.add(egui::DragValue::new(&mut vec.z).prefix(format!("{prefix}_2")));
    }

    fn create_vec2_for_mat2(ui: &mut egui::Ui, vec: &glam::Vec2, prefix: &str) {
        ui.add_enabled_ui(false, |ui| {
            let mut value = vec.x;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_0")));
            let mut value = vec.y;
            let _ = ui.add(egui::DragValue::new(&mut value).prefix(format!("{prefix}_1")));
        });
    }

    fn create_vec2_for_mat2_mut(ui: &mut egui::Ui, vec: &mut glam::Vec2, prefix: &str) {
        let _ = ui.add(egui::DragValue::new(&mut vec.x).prefix(format!("{prefix}_0")));
        let _ = ui.add(egui::DragValue::new(&mut vec.y).prefix(format!("{prefix}_1")));
    }

    impl ImguiPresentable for glam::Mat4 {
        fn render_component(&self, ui: &mut egui::Ui) {
            // if let Some(table) = ui.begin_table_header(
            //     "values",
            //     [
            //         imgui::TableColumnSetup::new("x_axis"),
            //         imgui::TableColumnSetup::new("y_axis"),
            //         imgui::TableColumnSetup::new("z_axis"),
            //         imgui::TableColumnSetup::new("w_axis"),
            //     ],
            // ) {
            //     ui.table_next_column();
            //     create_vec4_for_mat4(ui, &self.x_axis, "x");

            //     ui.table_next_column();
            //     create_vec4_for_mat4(ui, &self.y_axis, "y");

            //     ui.table_next_column();
            //     create_vec4_for_mat4(ui, &self.z_axis, "z");

            //     ui.table_next_column();
            //     create_vec4_for_mat4(ui, &self.w_axis, "w");

            //     table.end();
            // }
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 4);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x_axis");
                    });
                    header.col(|ui| {
                        ui.strong("y_axis");
                    });
                    header.col(|ui| {
                        ui.strong("z_axis");
                    });
                    header.col(|ui| {
                        ui.strong("w_axis");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| create_vec4_for_mat4(ui, &self.x_axis, "x"));
                        row.col(|ui| create_vec4_for_mat4(ui, &self.y_axis, "y"));
                        row.col(|ui| create_vec4_for_mat4(ui, &self.z_axis, "z"));
                        row.col(|ui| create_vec4_for_mat4(ui, &self.w_axis, "w"));
                    });
                });

            // if ui.button("Clipboard") {
            //     ui.set_clipboard_text(format!("{self:#?}"));
            // }

            // if ui.is_item_hovered() {
            //     ui.tooltip_text("Copies the debug representation to clipboard.");
            // }
        }

        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 4);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x_axis");
                    });
                    header.col(|ui| {
                        ui.strong("y_axis");
                    });
                    header.col(|ui| {
                        ui.strong("z_axis");
                    });
                    header.col(|ui| {
                        ui.strong("w_axis");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| create_vec4_for_mat4_mut(ui, &mut self.x_axis, "x"));
                        row.col(|ui| create_vec4_for_mat4_mut(ui, &mut self.y_axis, "y"));
                        row.col(|ui| create_vec4_for_mat4_mut(ui, &mut self.z_axis, "z"));
                        row.col(|ui| create_vec4_for_mat4_mut(ui, &mut self.w_axis, "w"));
                    });
                });

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("Identity")
                    .on_hover_text("Reset the matrix to an identity matrix.")
                    .clicked()
                {
                    *self = Self::IDENTITY;
                }

                if ui
                    .button("Zero")
                    .on_hover_text("Reset the matrix to a zero matrix.")
                    .clicked()
                {
                    *self = Self::ZERO;
                }

                if ui
                    .button("NaN")
                    .on_hover_text("Reset the matrix elements to f32::NaN.")
                    .clicked()
                {
                    *self = Self::NAN;
                }

                if ui
                    .button("Clipboard")
                    .on_hover_text("Copies the debug representation to clipboard.")
                    .clicked()
                {
                    // not implemented since egui doesn't have a clipboard API.
                }
            });
        }
    }

    impl ImguiPresentable for glam::Mat3 {
        fn render_component(&self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 3);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x_axis");
                    });
                    header.col(|ui| {
                        ui.strong("y_axis");
                    });
                    header.col(|ui| {
                        ui.strong("z_axis");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| create_vec3_for_mat3(ui, &self.x_axis, "x"));
                        row.col(|ui| create_vec3_for_mat3(ui, &self.y_axis, "y"));
                        row.col(|ui| create_vec3_for_mat3(ui, &self.z_axis, "z"));
                    });
                });

            // if ui.button("Clipboard") {
            //     ui.set_clipboard_text(format!("{self:#?}"));
            // }

            // if ui.is_item_hovered() {
            //     ui.tooltip_text("Copies the debug representation to clipboard.");
            // }
        }

        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 3);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x_axis");
                    });
                    header.col(|ui| {
                        ui.strong("y_axis");
                    });
                    header.col(|ui| {
                        ui.strong("z_axis");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| create_vec3_for_mat3_mut(ui, &mut self.x_axis, "x"));
                        row.col(|ui| create_vec3_for_mat3_mut(ui, &mut self.y_axis, "y"));
                        row.col(|ui| create_vec3_for_mat3_mut(ui, &mut self.z_axis, "z"));
                    });
                });

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("Identity")
                    .on_hover_text("Reset the matrix to an identity matrix.")
                    .clicked()
                {
                    *self = Self::IDENTITY;
                }

                if ui
                    .button("Zero")
                    .on_hover_text("Reset the matrix to a zero matrix.")
                    .clicked()
                {
                    *self = Self::ZERO;
                }

                if ui
                    .button("NaN")
                    .on_hover_text("Reset the matrix elements to f32::NaN.")
                    .clicked()
                {
                    *self = Self::NAN;
                }

                if ui
                    .button("Clipboard")
                    .on_hover_text("Copies the debug representation to clipboard.")
                    .clicked()
                {
                    // not implemented since egui doesn't have a clipboard API.
                }
            });
        }
    }

    impl ImguiPresentable for glam::Mat2 {
        fn render_component(&self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 2);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x_axis");
                    });
                    header.col(|ui| {
                        ui.strong("y_axis");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| create_vec2_for_mat2(ui, &self.x_axis, "x"));
                        row.col(|ui| create_vec2_for_mat2(ui, &self.y_axis, "y"));
                    });
                });

            // if ui.button("Clipboard") {
            //     ui.set_clipboard_text(format!("{self:#?}"));
            // }

            // if ui.is_item_hovered() {
            //     ui.tooltip_text("Copies the debug representation to clipboard.");
            // }
        }

        fn render_component_mut(&mut self, ui: &mut egui::Ui) {
            let table = TableBuilder::new(ui)
                .striped(true)
                .columns(Column::auto(), 2);

            table
                .header(20.0f32, |mut header| {
                    header.col(|ui| {
                        ui.strong("x_axis");
                    });
                    header.col(|ui| {
                        ui.strong("y_axis");
                    });
                })
                .body(|mut body| {
                    body.row(20.0f32, |mut row| {
                        // ui.separator();
                        row.col(|ui| create_vec2_for_mat2_mut(ui, &mut self.x_axis, "x"));
                        row.col(|ui| create_vec2_for_mat2_mut(ui, &mut self.y_axis, "y"));
                    });
                });

            ui.horizontal_wrapped(|ui| {
                if ui
                    .button("Identity")
                    .on_hover_text("Reset the matrix to an identity matrix.")
                    .clicked()
                {
                    *self = Self::IDENTITY;
                }

                if ui
                    .button("Zero")
                    .on_hover_text("Reset the matrix to a zero matrix.")
                    .clicked()
                {
                    *self = Self::ZERO;
                }

                if ui
                    .button("NaN")
                    .on_hover_text("Reset the matrix elements to f32::NaN.")
                    .clicked()
                {
                    *self = Self::NAN;
                }

                if ui
                    .button("Clipboard")
                    .on_hover_text("Copies the debug representation to clipboard.")
                    .clicked()
                {
                    // not implemented since egui doesn't have a clipboard API.
                }
            });
        }
    }
}

#[cfg(feature = "egui_backend")]
pub use egui_backend::*;

//! This module provides impementations of the [ImguiPresentable] trait
//! for the commonly used std library types of Rust.

use crate::{Extent, ImguiPresentable};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

impl<T: ImguiPresentable> ImguiPresentable for Vec<T> {
    fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));
        if let Some(table) = ui.begin_table_header(
            "objects",
            [imgui::TableColumnSetup::new(&format!(
                "objects ({type_name}): {}",
                self.len()
            ))],
        ) {
            ui.table_next_column();

            self.iter().enumerate().for_each(|(i, o)| {
                ui.separator();
                ui.tree_node_config(&format!("{i}: {type_name}##{o:p}"))
                    .opened(false, imgui::Condition::FirstUseEver)
                    .framed(true)
                    .build(|| {
                        o.render_component(ui, extent);
                    });
            });

            table.end();
        }
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));
        // let filter = ui.input_scalar("Filter index", )

        if let Some(table) = ui.begin_table_header_with_flags(
            "objects",
            [imgui::TableColumnSetup::new(&format!(
                "objects ({type_name}): {}",
                self.len()
            ))],
            imgui::TableFlags::BORDERS | imgui::TableFlags::ROW_BG,
        ) {
            ui.table_next_column();

            let mut to_delete = None;
            self.iter_mut().enumerate().for_each(|(i, o)| {
                let mut is_not_deleted = true;
                if ui.collapsing_header_with_close_button(
                    format!("{i}: {type_name}##{o:p}"),
                    imgui::TreeNodeFlags::FRAMED,
                    &mut is_not_deleted,
                ) {
                    ui.indent();
                    o.render_component_mut(ui, extent);
                    ui.unindent();
                }

                if !is_not_deleted {
                    to_delete = Some(i);
                }
            });

            if let Some(index) = to_delete {
                let _ = self.remove(index);
            }

            table.end();
        }

        // if ui.button("Add new") {
        //     self.push()
        // }

        // if ui.is_item_hovered() {
        //     ui.tooltip_text("Adds a new element to the end of the vector by cloning the last object in it.");
        // }

        // ui.same_line();

        if ui.button("Clear") {
            self.clear();
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Clears the vector.");
        }
    }
}

impl<T: ImguiPresentable + Ord> ImguiPresentable for BTreeSet<T> {
    fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));

        if let Some(table) = ui.begin_table_header(
            "objects",
            [imgui::TableColumnSetup::new(&format!(
                "objects ({type_name}): {}",
                self.len()
            ))],
        ) {
            ui.table_next_column();

            self.iter().for_each(|o| {
                o.render_component(ui, extent);
            });

            table.end();
        }
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));

        if let Some(table) = ui.begin_table_header(
            "objects",
            [imgui::TableColumnSetup::new(&format!(
                "objects ({type_name}): {}",
                self.len()
            ))],
        ) {
            ui.table_next_column();

            // let to_delete = self.iter().enumerate().find(|(i, o)| {
            //     let mut is_not_deleted = true;
            //     if ui.collapsing_header_with_close_button(
            //         format!("{i}: {type_name}##{:p}", std::ptr::addr_of!(o)),
            //         imgui::TreeNodeFlags::FRAMED,
            //         &mut is_not_deleted,
            //     ) {
            //         ui.indent();
            //         o.render_component(ui, extent);
            //         ui.unindent();
            //     }

            //     !is_not_deleted
            // });

            // if let Some(object) = to_delete {
            //     let _ = self.remove(object.1);
            // }

            let mut index = 0usize;
            self.retain(|o| {
                let mut is_not_deleted = true;
                if ui.collapsing_header_with_close_button(
                    format!("{index}: {type_name}##{o:p}"),
                    imgui::TreeNodeFlags::FRAMED,
                    &mut is_not_deleted,
                ) {
                    ui.indent();
                    o.render_component(ui, extent);
                    ui.unindent();
                }

                index += 1;

                is_not_deleted
            });

            table.end();
        }

        if ui.button("Clear") {
            self.clear();
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Clears the set.");
        }
    }
}

impl<T: ImguiPresentable> ImguiPresentable for HashSet<T> {
    fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));

        if let Some(table) = ui.begin_table_header(
            "objects",
            [imgui::TableColumnSetup::new(&format!(
                "objects ({type_name}): {}",
                self.len()
            ))],
        ) {
            ui.table_next_column();

            self.iter().for_each(|o| {
                o.render_component(ui, extent);
            });

            table.end();
        }
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));

        if let Some(table) = ui.begin_table_header(
            "objects",
            [imgui::TableColumnSetup::new(&format!(
                "objects ({type_name}): {}",
                self.len()
            ))],
        ) {
            ui.table_next_column();

            let mut index = 0usize;
            self.retain(|o| {
                let mut is_not_deleted = true;
                if ui.collapsing_header_with_close_button(
                    format!("{index}: {type_name}##{o:p}"),
                    imgui::TreeNodeFlags::FRAMED,
                    &mut is_not_deleted,
                ) {
                    ui.indent();
                    o.render_component(ui, extent);
                    ui.unindent();
                }

                index += 1;

                is_not_deleted
            });

            table.end();
        }

        if ui.button("Clear") {
            self.clear();
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Clears the set.");
        }
    }
}

impl<K: ImguiPresentable, V: ImguiPresentable> ImguiPresentable for BTreeMap<K, V> {
    fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
        let _id = ui.push_id(&format!("##{self:p}"));
        if let Some(table) = ui.begin_table_header(
            "objects",
            [
                imgui::TableColumnSetup::new("#"),
                imgui::TableColumnSetup::new("key"),
                imgui::TableColumnSetup::new("value"),
            ],
        ) {
            self.iter().enumerate().for_each(|(i, (k, v))| {
                ui.table_next_column();
                i.render_component(ui, extent);

                ui.table_next_column();
                k.render_component(ui, extent);

                ui.table_next_column();
                v.render_component(ui, extent);
            });

            table.end();
        }
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        let _id = ui.push_id(&format!("##{self:p}"));

        if let Some(table) = ui.begin_table_header(
            "objects",
            [
                imgui::TableColumnSetup::new("#"),
                imgui::TableColumnSetup::new("key"),
                imgui::TableColumnSetup::new("value"),
            ],
        ) {
            self.iter_mut().enumerate().for_each(|(i, (k, v))| {
                ui.table_next_column();
                i.render_component(ui, extent);

                ui.table_next_column();
                k.render_component(ui, extent);

                ui.table_next_column();
                v.render_component_mut(ui, extent);
            });

            table.end();
        }

        if ui.button(format!("Clear##{self:p}")) {
            self.clear();
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Clears the map.");
        }
    }
}

impl<K: ImguiPresentable, V: ImguiPresentable> ImguiPresentable for HashMap<K, V> {
    fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
        let _id = ui.push_id(&format!("##{self:p}"));
        if let Some(table) = ui.begin_table_header(
            "objects",
            [
                imgui::TableColumnSetup::new("#"),
                imgui::TableColumnSetup::new("key"),
                imgui::TableColumnSetup::new("value"),
            ],
        ) {
            self.iter().enumerate().for_each(|(i, (k, v))| {
                ui.table_next_column();
                i.render_component(ui, extent);

                ui.table_next_column();
                k.render_component(ui, extent);

                ui.table_next_column();
                v.render_component(ui, extent);
            });

            table.end();
        }
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        let _id = ui.push_id(&format!("##{self:p}"));

        if let Some(table) = ui.begin_table_header(
            "objects",
            [
                imgui::TableColumnSetup::new("#"),
                imgui::TableColumnSetup::new("key"),
                imgui::TableColumnSetup::new("value"),
            ],
        ) {
            self.iter_mut().enumerate().for_each(|(i, (k, v))| {
                ui.table_next_column();
                i.render_component(ui, extent);

                ui.table_next_column();
                k.render_component(ui, extent);

                ui.table_next_column();
                v.render_component_mut(ui, extent);
            });

            table.end();
        }

        if ui.button(format!("Clear##{self:p}")) {
            self.clear();
        }

        if ui.is_item_hovered() {
            ui.tooltip_text("Clears the map.");
        }
    }
}

impl<T: ImguiPresentable + Default> ImguiPresentable for Option<T> {
    fn render_component(&self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));

        let mut has_value = self.is_some();

        ui.disabled(true, || {
            let checked = ui.checkbox(format!("Has value ({type_name})##{self:p}"), &mut has_value);
            if checked || has_value {
                match self.as_ref() {
                    Some(value) => value.render_component(ui, extent),
                    None => {
                        let temp = T::default();
                        temp.render_component(ui, extent);
                    }
                }
            }
        });
    }

    fn render_component_mut(&mut self, ui: &imgui::Ui, extent: Extent) {
        let type_name = std::any::type_name::<T>();
        let _id = ui.push_id(&format!("##{self:p}"));
        let had_value = self.is_some();
        let mut has_value = had_value;
        let checked = ui.checkbox(
            &format!("Has value ({type_name})##{self:p}"),
            &mut has_value,
        );
        if checked || has_value {
            if !had_value {
                let _ = self.insert(T::default());
            } else if !has_value {
                let _ = self.take();
            }

            if let Some(value) = self.as_mut() {
                value.render_component_mut(ui, extent);
            }
        }
    }
}

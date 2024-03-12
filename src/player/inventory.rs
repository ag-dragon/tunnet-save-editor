use bevy_egui::egui;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Inventory {
    items: HashMap<String, i32>,
}

#[derive(EnumIter, Clone, Copy, Debug)]
pub enum InventoryItem {
    Battery,
    Scrap,
    Toy,
    Screw,
    Disk,
    Corn,
    Speaker,
    Seeds,
    Magnet,
    Oil,
}

pub fn inventory_editor(ui: &mut egui::Ui, inventory: &mut Inventory) {
    ui.collapsing("Inventory", |ui| {
        for item in InventoryItem::iter() {
            let mut item_count: i32 = 0;
            match inventory.items.get(&format!("{:?}", item).to_lowercase()) {
                Some(count) => {
                    item_count = *count;
                },
                _ => {},
            }
            ui.horizontal(|ui| {
                ui.label(format!("{:?}", item));
                ui.add(egui::DragValue::new(&mut item_count).clamp_range(0..=99).speed(1));
            });
            if item_count > 0 {
                inventory.items.insert(format!("{:?}", item).to_lowercase(), item_count);
            } else if item_count == 0 {
                inventory.items.remove(&format!("{:?}", item).to_lowercase());
            }
        }
    });
}

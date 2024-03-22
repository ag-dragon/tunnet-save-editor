use tunnet_save::player::home::{Home, Furniture};

use bevy_egui::egui;

pub fn home_editor(ui: &mut egui::Ui, home: &mut Home) {
    ui.collapsing("Home", |ui| {
        ui.collapsing("Items", |ui| {
            for item in Furniture::iter() {
                ui.horizontal(|ui| {
                    let mut furniture = home.items.contains(&item);
                    ui.checkbox(&mut furniture, format!("{:?}", item));
                    if !furniture && home.items.contains(&item) {
                        home.items.retain(|&x| x != item);
                    } else if furniture && !home.items.contains(&item) {
                        home.items.push(item);
                    }
                });
            }
        });
    });
}

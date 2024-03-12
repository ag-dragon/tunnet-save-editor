pub mod story;
pub mod shop;
pub mod inventory;
pub mod journal;
pub mod home;
pub mod guide;

use crate::save_file::SaveFile;

use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Player {
    pos: [f64; 3],
    credits: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PlayerMovement {
    movement: bool,
    look: bool,
    sprint: bool,
}

pub fn player_editor(ui: &mut egui::Ui, save_file: &mut SaveFile) {
    ui.vertical_centered(|ui| {
        ui.heading("Player");
    });


    ui.collapsing("Player Data", |ui| {
        ui.collapsing("Player Position", |ui| {
            ui.horizontal(|ui| {
                let mut px = save_file.player.pos[0];
                ui.add(egui::DragValue::new(&mut px).speed(0.1).prefix("x: "));
                save_file.player.pos[0] = px;
                let mut py = save_file.player.pos[1];
                ui.add(egui::DragValue::new(&mut py).speed(0.1).prefix("y: "));
                save_file.player.pos[1] = py;
                let mut pz = save_file.player.pos[2];
                ui.add(egui::DragValue::new(&mut pz).speed(0.1).prefix("z: "));
                save_file.player.pos[2] = pz;
            });
        });
        ui.horizontal(|ui| {
            ui.label("Credits:");
            let mut credits = save_file.player.credits;
            ui.add(egui::DragValue::new(&mut credits).speed(0.1));
            save_file.player.credits = credits;
        });
        ui.horizontal(|ui| {
            let mut movement = save_file.story.player_movement.movement;
            ui.checkbox(&mut movement, "Can Walk/Sprint");
            save_file.story.player_movement.movement = movement;
        });
        ui.horizontal(|ui| {
            let mut look = save_file.story.player_movement.look;
            ui.checkbox(&mut look, "Can Move Camera");
            save_file.story.player_movement.look = look;
        });
    });


    shop::shop_editor(ui, &mut save_file.story.shop);
    inventory::inventory_editor(ui, &mut save_file.story.inventory);
    journal::journal_editor(ui, &mut save_file.story.knowledge);
    home::home_editor(ui, &mut save_file.story.home);
    guide::guide_editor(ui, &mut save_file.story.guide);
}

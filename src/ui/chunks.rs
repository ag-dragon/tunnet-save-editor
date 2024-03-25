use crate::chunk_editor::GenBlockMeshEvent;
use crate::CurrentChunk;

use bevy::prelude::*;
use bevy_egui::egui;

pub fn chunk_editor(
    ui: &mut egui::Ui,
    mut ev_genblockmesh: EventWriter<GenBlockMeshEvent>,
    mut current_chunk: ResMut<CurrentChunk>,
) {
    ui.vertical_centered(|ui| {
        ui.heading("Chunks");

        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new(&mut current_chunk.0.x).speed(1));
            ui.add(egui::DragValue::new(&mut current_chunk.0.y).speed(1));
            ui.add(egui::DragValue::new(&mut current_chunk.0.z).speed(1));
            if ui.button("Select Chunk").clicked() {
                ev_genblockmesh.send(GenBlockMeshEvent);
            }
        });
    });
}

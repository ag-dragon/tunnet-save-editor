pub mod map;
pub mod rooms;
pub mod voxels;

use rooms::ChunkType;
use crate::CurrentChunk;
use crate::chunk_renderer::GenBlockMeshEvent;

use bevy::prelude::*;
use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ChunkCoords {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        ChunkCoords {
            x,
            y,
            z,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Chunk {
    Coords(ChunkCoords),
    Data(Vec<[i32; 2]>),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Chunks {
    chunk_types: Vec<Vec<ChunkType>>,
    pub chunks: Vec<Vec<Chunk>>,
}

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

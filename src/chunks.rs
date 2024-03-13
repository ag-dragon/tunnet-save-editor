pub mod map;
pub mod rooms;
pub mod voxels;

use rooms::ChunkType;

use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChunkCoords {
    x: i32,
    y: i32,
    z: i32,
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
    chunks: Vec<Vec<Chunk>>,
}

pub fn chunk_editor(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Chunks");
    });
}

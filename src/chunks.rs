use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChunkCoords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum MapColor {
    #[default]
    Pink,
    Yellow,
    Green,
    Blue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Annotation {
    Coords(ChunkCoords),
    Description { color: MapColor, note: String },
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MapAnnotations {
    annotations: Vec<[Annotation; 2]>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Map {
    visited_chunks: Vec<ChunkCoords>,
    map_annotations: MapAnnotations,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChunkRoom {
    CloisterCeiling,
    SwimmingPool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ChunkType {
    Coords(ChunkCoords),
    Rooms {
        BunkerRoom: [Option<ChunkRoom>; 3],
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Chunk {
    Coords(ChunkCoords),
    Data(Vec<[i32; 2]>),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChunkData {
    chunk_types: Vec<Vec<ChunkType>>,
    chunks: Vec<Vec<Chunk>>,
}

pub fn chunk_editor(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Chunks");
    });
}

use serde::{Serialize, Deserialize};

use crate::chunks::ChunkCoords;

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

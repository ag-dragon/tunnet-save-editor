pub mod map;
pub mod rooms;

use rooms::ChunkType;
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

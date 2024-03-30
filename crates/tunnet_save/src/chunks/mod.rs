pub mod map;
pub mod rooms;

use rooms::ChunkType;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Default, Debug)]
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

#[serde_as]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Chunks {
    pub chunk_types: Vec<Vec<ChunkType>>,
    #[serde_as(as = "Vec<(_, _)>")]
    pub chunks: HashMap<ChunkCoords, Vec<[i32; 2]>>,
}

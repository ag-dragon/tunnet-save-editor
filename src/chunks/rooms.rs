use serde::{Serialize, Deserialize};

use crate::chunks::ChunkCoords;

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
        #[serde(rename = "BunkerRoom")]
        bunker_room: [Option<ChunkRoom>; 3],
    },
}

use serde::{Serialize, Deserialize};

use crate::chunks::ChunkCoords;
use crate::network::Address;

#[derive(Serialize, Deserialize, Debug)]
pub enum ChunkRoom {
    CloisterCeiling,
    SwimmingPool,
    MainframeRoom(Address),
    CommandRoom(Address),
    Church(Address),
    MainframeRoomLuxury(Address),
    FilterLab(Address),
    HubLab(Address),
    AntennaLab(Address),
    Hermit(Address),
    School(Address),
    PartyShelter(Address),
    TesterLab(Address),
    Farm(Address),
    Elderlies(Address),
    PoorShelter(Address),
    ServerRoom(Address),
    Workshop(Address),
    MilitaryOutpost(Address),
    Beach(Address),
    Cave(Address),
    Supermarket(Address),
    Shelter(Address),
    Camp(Address),
    MainframeRoomAbandonedLab(Address),
    AbandonedLabNorthWest(Address),
    AbandonedLabNorthEast(Address),
    AbandonedLabSouthWest(Address),
    AbandonedLabSouthEast(Address),

    #[serde(other)]
    Unknown,
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

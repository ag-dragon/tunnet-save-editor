use crate::player::Player;
use crate::player::story::Story;
use crate::network::Network;

use bevy::ecs::system::Resource;
use serde::{Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct SaveFile {
    pub player: Player,
    pub story: Story,
    #[serde(flatten)]
    pub network: Network,
}

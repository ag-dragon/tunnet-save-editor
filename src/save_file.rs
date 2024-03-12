use crate::player::Player;
use crate::player::story::Story;

use bevy::ecs::system::Resource;
use serde::{Serialize, Deserialize};

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
pub struct SaveFile {
    pub player: Player,
    pub story: Story,
}

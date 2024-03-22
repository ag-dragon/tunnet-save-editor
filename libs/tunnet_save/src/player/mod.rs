pub mod story;
pub mod shop;
pub mod inventory;
pub mod journal;
pub mod home;
pub mod guide;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Player {
    pub pos: [f64; 3],
    pub credits: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PlayerMovement {
    pub movement: bool,
    pub look: bool,
    pub sprint: bool,
}

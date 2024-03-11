use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Inventory {
    pub items: HashMap<String, i32>,
}

#[derive(EnumIter, Clone, Copy, Debug)]
pub enum InventoryItem {
    Battery,
    Scrap,
    Toy,
    Screw,
    Disk,
    Corn,
    Speaker,
    Seeds,
    Magnet,
    Oil,
}

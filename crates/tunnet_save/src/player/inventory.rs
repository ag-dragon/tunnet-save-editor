use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Inventory {
    pub items: HashMap<String, i32>,
}

#[derive(Clone, Copy, Debug)]
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

impl InventoryItem {
    pub fn iter() -> impl Iterator<Item = InventoryItem> {
        use InventoryItem::*;
        [
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
        ].iter().copied()
    }
}

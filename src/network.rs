use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum BaseFour {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum AddressType {
    Endpoint,
    Filter,
    UnrestrictedFilter,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    elements: Vec<BaseFour>,
    address_type: AddressType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AddressOrVec {
    A(Address),
    V(Vec<Address>),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NetworkNodes {
}

pub fn network_editor(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Network");
    });
}

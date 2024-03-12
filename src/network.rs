use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum BaseFour {
    Zero,
    One,
    Two,
    Three,
    Wildcard,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pos: [f64; 3],
    up: [f64; 3],
    angle: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Infection {
    Hack,
    Bio,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Endpoint {
    node: i32,
    address: Address,
    infection: Option<Infection>,
    disinfection: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relay {
    node: i32,
    fixed: bool,
    light: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterAddr {
    Dst,
    Src,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterAction {
    DropPacket,
    SendBackPacket,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterOp {
    Match,
    Differ,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterCollision {
    DropInbound,
    DropOutbound,
    SendBackOutbound,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FilterConfig {
    port: i32,
    mask: Address,
    addr: FilterAddr,
    action: FilterAction,
    op: FilterOp,
    collision: FilterCollision,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    node: i32,
    config: FilterConfig,
    fixed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TesterMode {
    Single,
    Repeat,
    Snoop,
    Spoof,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TesterConfig {
    address: Address,
    destination: Address,
    mode: TesterMode,
    freq: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tester {
    node: i32,
    config: TesterConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hub {
    node: i32,
    fixed: bool,
    dir: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Antenna {
    node: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bridge {
    node: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Network {
    nodes: Vec<Node>,
    edges: Vec<Vec<[i32; 2]>>,
    endpoints: Vec<Endpoint>,
    relays: Vec<Relay>,
    filters: Vec<Filter>,
    testers: Vec<Tester>,
    hubs: Vec<Hub>,
    antennas: Vec<Antenna>,
    bridges: Vec<Bridge>,
}

pub fn network_editor(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Network");
    });
}

use bevy_egui::egui;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Shop {
    shop_level: i32,
    digging: bool,
    relay: bool,
    hub: bool,
    filter: bool,
    scan_short: bool,
    scan_long: bool,
    jetpack: bool,
    antivirus: bool,
    optical_fiber: bool,
    antenna: bool,
    tester: bool,
    relay_light: bool,
    patch: bool, //?
    filter_collision: bool,
    filter_full_address: bool,
    tester_repeat: bool,
    tester_spoof: bool,
    tester_snoop: bool,
    scan_short_enhanced: bool,
    scan_long_peers: bool,
    auto_map: bool,
}

pub fn shop_editor(ui: &mut egui::Ui, shop: &mut Shop) {
    ui.collapsing("Shop Unlocks", |ui| {
        ui.horizontal(|ui| {
            let mut drill = shop.digging;
            ui.checkbox(&mut drill, "Drill");
            shop.digging = drill;
        });
        ui.horizontal(|ui| {
            let mut patch = shop.patch;
            ui.checkbox(&mut patch, "Drill Patching");
            shop.patch = patch;
        });
        ui.horizontal(|ui| {
            let mut relay = shop.relay;
            ui.checkbox(&mut relay, "Relay");
            shop.relay = relay;
        });
        ui.horizontal(|ui| {
            let mut relay_light = shop.relay_light;
            ui.checkbox(&mut relay_light, "Relay Light");
            shop.relay_light = relay_light;
        });
        ui.horizontal(|ui| {
            let mut hub = shop.hub;
            ui.checkbox(&mut hub, "Hub");
            shop.hub = hub;
        });
        ui.horizontal(|ui| {
            let mut filter = shop.filter;
            ui.checkbox(&mut filter, "Filter");
            shop.filter = filter;
        });
        ui.horizontal(|ui| {
            let mut filter_collision = shop.filter_collision;
            ui.checkbox(&mut filter_collision, "Filter Collision Handling");
            shop.filter_collision = filter_collision;
        });
        ui.horizontal(|ui| {
            let mut filter_full_address = shop.filter_full_address;
            ui.checkbox(&mut filter_full_address, "Filter Full Address");
            shop.filter_full_address = filter_full_address;
        });
        ui.horizontal(|ui| {
            let mut scan_short = shop.scan_short;
            ui.checkbox(&mut scan_short, "Short-Range Rader");
            shop.scan_short = scan_short;
        });
        ui.horizontal(|ui| {
            let mut scan_short_enhanced = shop.scan_short_enhanced;
            ui.checkbox(&mut scan_short_enhanced, "Short-Range Rader Enhanced");
            shop.scan_short_enhanced = scan_short_enhanced;
        });
        ui.horizontal(|ui| {
            let mut scan_long = shop.scan_long;
            ui.checkbox(&mut scan_long, "Long-Range Radar");
            shop.scan_long = scan_long;
        });
        ui.horizontal(|ui| {
            let mut scan_long_peers = shop.scan_long_peers;
            ui.checkbox(&mut scan_long_peers, "Long-Range Radar Peers");
            shop.scan_long_peers = scan_long_peers;
        });
        ui.horizontal(|ui| {
            let mut antivirus = shop.antivirus;
            ui.checkbox(&mut antivirus, "Antivirus");
            shop.antivirus = antivirus;
        });
        ui.horizontal(|ui| {
            let mut tester = shop.tester;
            ui.checkbox(&mut tester, "Tester");
            shop.tester = tester;
        });
        ui.horizontal(|ui| {
            let mut tester_repeat = shop.tester_repeat;
            ui.checkbox(&mut tester_repeat, "Tester Repeat");
            shop.tester_repeat = tester_repeat;
        });
        ui.horizontal(|ui| {
            let mut tester_spoof = shop.tester_spoof;
            ui.checkbox(&mut tester_spoof, "Tester Spoof");
            shop.tester_spoof = tester_spoof;
        });
        ui.horizontal(|ui| {
            let mut tester_snoop = shop.tester_snoop;
            ui.checkbox(&mut tester_snoop, "Tester Snoop");
            shop.tester_snoop = tester_snoop;
        });
        ui.horizontal(|ui| {
            let mut jetpack = shop.jetpack;
            ui.checkbox(&mut jetpack, "Jetpack");
            shop.jetpack = jetpack;
        });
        ui.horizontal(|ui| {
            let mut auto_map = shop.auto_map;
            ui.checkbox(&mut auto_map, "Auto-Map");
            shop.auto_map = auto_map;
        });
    });
}

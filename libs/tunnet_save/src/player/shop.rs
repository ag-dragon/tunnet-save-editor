use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Shop {
    pub shop_level: i32,
    pub digging: bool,
    pub relay: bool,
    pub hub: bool,
    pub filter: bool,
    pub scan_short: bool,
    pub scan_long: bool,
    pub jetpack: bool,
    pub antivirus: bool,
    pub optical_fiber: bool,
    pub antenna: bool,
    pub tester: bool,
    pub relay_light: bool,
    pub patch: bool, //?
    pub filter_collision: bool,
    pub filter_full_address: bool,
    pub tester_repeat: bool,
    pub tester_spoof: bool,
    pub tester_snoop: bool,
    pub scan_short_enhanced: bool,
    pub scan_long_peers: bool,
    pub auto_map: bool,
}

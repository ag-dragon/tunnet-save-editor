use bevy_egui::egui;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Home {
    pub items: Vec<Furniture>,
    pub ads: Vec<Ad>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum Furniture {
    PalmTree,
    RoundTree,
    Shrub,
    Oilcan,
    Jar,
    Server,
    LargeBed,
    MedicalBed,
    RoundTable,
    VegetableBed,
    VendingMachine,
    WashingMachine,
    DjTable,
    CreditDispenser,
    LifeBuoy,
    Playground,
    Open,
    NoticeBoard,
    Symbol,
    DoNotEnter,
    RestrictedArea,
    VIP,
    DiggingMachine,
    BookYoursNow,
    EmployeesOnly,
    Xrays,
    SecurityCheck,
    WorldMap,
    HaveFaith,
    WearHelmet,
    WeWantYou,
    Farmers,
    Fields,
    HomeSweetHome,
    Grandson,
    Construction,
    Swim,
    WashYourHands,
    Bevy,
    Ferries,
    Opcodes,
    Alphabet,
    FakeWindows,
    NightClubArt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ad {
    pub id: i64,
    pub title: String,
    pub price: i32,
    pub seller: crate::Address,
    pub qty: i32,
    pub item: String,
}

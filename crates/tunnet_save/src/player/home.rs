use crate::network::Address;

use serde::{Serialize, Deserialize};

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

impl Furniture {
    pub fn iter() -> impl Iterator<Item = Furniture> {
        use Furniture::*;
        [
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
        ].iter().copied()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ad {
    id: i64,
    title: String,
    price: i32,
    seller: Address,
    qty: i32,
    item: String,
}

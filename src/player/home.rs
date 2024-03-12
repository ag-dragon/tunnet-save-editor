use bevy_egui::egui;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Home {
    items: Vec<Furniture>,
    ads: Vec<Ad>,
}

#[derive(EnumIter, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
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
    id: i64,
    title: String,
    price: i32,
    seller: crate::Address,
    qty: i32,
    item: String,
}

pub fn home_editor(ui: &mut egui::Ui, home: &mut Home) {
    ui.collapsing("Home", |ui| {
        ui.collapsing("Items", |ui| {
            for item in Furniture::iter() {
                ui.horizontal(|ui| {
                    let mut furniture = home.items.contains(&item);
                    ui.checkbox(&mut furniture, format!("{:?}", item));
                    if !furniture && home.items.contains(&item) {
                        home.items.retain(|&x| x != item);
                    } else if furniture && !home.items.contains(&item) {
                        home.items.push(item);
                    }
                });
            }
        });
    });
}

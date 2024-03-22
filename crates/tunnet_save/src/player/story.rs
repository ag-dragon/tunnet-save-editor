use super::shop::Shop;
use super::inventory::Inventory;
use super::journal::Knowledge;
use super::home::Home;
use super::guide::Guide;
use crate::player::PlayerMovement;
use crate::network::AddressOrVec;
use crate::chunks::map::Map;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Default, Debug)]
pub enum BossPhase {
    #[default]
    InfectMainframeWithAB,
    InfectMainframeWithABCD,
    KillswitchOne,
    KillswitchTwo,
    KillswitchThree,
    KillswitchFour,
    DisinfectEndpointOne,
    DisinfectEndpointTwo,
    DisinfectEndpointThree,
    DisinfectEndpointFour,
    DisinfectMainframe,
    Destroyed,
}

impl BossPhase {
    pub fn iter() -> impl Iterator<Item = BossPhase> {
        use BossPhase::*;
        [
            InfectMainframeWithAB,
            InfectMainframeWithABCD,
            KillswitchOne,
            KillswitchTwo,
            KillswitchThree,
            KillswitchFour,
            DisinfectEndpointOne,
            DisinfectEndpointTwo,
            DisinfectEndpointThree,
            DisinfectEndpointFour,
            DisinfectMainframe,
            Destroyed,
        ].iter().copied()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Default, Debug)]
pub enum StoryState {
    BootUp,
    Setup,
    PressButton,
    ConnectRelayToMainframe,
    ConnectEndpointToMainframe,
    UseHub,
    UseScan,
    ConnectToShelters,
    TalkToScientist,
    DisinfectMainframe,
    DisinfectOtherMainframes,
    FindAbandonedLab,
    DestroyCorruptedMainframe,
    GoToSurface,
    #[default]
    TheEnd,
    Review,
    DemoEnd,
}

impl StoryState {
    pub fn iter() -> impl Iterator<Item = StoryState> {
        use StoryState::*;
        [
            BootUp,
            Setup,
            PressButton,
            ConnectRelayToMainframe,
            ConnectEndpointToMainframe,
            UseHub,
            UseScan,
            ConnectToShelters,
            TalkToScientist,
            DisinfectMainframe,
            DisinfectOtherMainframes,
            FindAbandonedLab,
            DestroyCorruptedMainframe,
            GoToSurface,
            TheEnd,
            Review,
            DemoEnd,
        ].iter().copied()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Story {
    pub state: StoryState,
    pub boss_phase: BossPhase,
    pub companion: bool,
    pub surface: bool,
    pub review: bool,
    pub disinfected: i32,
    pub disinfection_dialog: i32,
    pub military_cleared: bool,
    pub luxury_cleared: bool,
    pub monastry_cleared: bool,
    pub researchlab_cleared: bool,

    #[serde(flatten)]
    pub shop: Shop,

    #[serde(flatten)]
    pub player_movement: PlayerMovement,

    #[serde(flatten)]
    pub guide: Guide,

    pub inventory: Inventory,

    pub knowledge: Knowledge,

    pub home: Home,

    #[serde(flatten)]
    pub map: Map,

    pub connection_status: Vec<Vec<AddressOrVec>>,

    pub streaks: Vec<i32>, // Placeholder: I've never seen streaks contain any data
    pub mainframes: Vec<i32>, // Placeholder: I've never seen mainframes contain any data
}

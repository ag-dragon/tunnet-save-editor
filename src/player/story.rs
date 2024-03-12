use crate::{ChunkCoords, MapAnnotations};
use super::shop::Shop;
use super::inventory::Inventory;
use super::journal::Knowledge;
use super::home::Home;
use super::guide::Guide;
use crate::player::PlayerMovement;

use bevy_egui::egui;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Serialize, Deserialize, PartialEq, Clone, Copy, Default, Debug)]
enum BossPhase {
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

#[derive(EnumIter, Serialize, Deserialize, PartialEq, Clone, Copy, Default, Debug)]
enum StoryState {
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Story {
    state: StoryState,
    boss_phase: BossPhase,
    companion: bool,
    surface: bool,
    review: bool,
    disinfected: i32,
    disinfection_dialog: i32,
    military_cleared: bool,
    luxury_cleared: bool,
    monastry_cleared: bool,
    researchlab_cleared: bool,

    #[serde(flatten)]
    pub shop: Shop,

    #[serde(flatten)]
    pub player_movement: PlayerMovement,

    #[serde(flatten)]
    pub guide: Guide,

    pub inventory: Inventory,

    pub knowledge: Knowledge,

    pub home: Home,

    pub visited_chunks: Vec<ChunkCoords>,
    pub map_annotations: MapAnnotations,

    //connection_status: Vec
    // streaks?
    // mainframes: Vec?
}

pub fn story_editor(ui: &mut egui::Ui, story: &mut Story) {
    ui.collapsing("Story Progress", |ui| {
        let mut story_selected = story.state;
        egui::ComboBox::from_label("Story State")
                .selected_text(format!("{:?}", story_selected))
                .show_ui(ui, |ui| {
            for story_state in StoryState::iter() {
                ui.selectable_value(&mut story_selected, story_state, format!("{:?}", story_state));
            }
        });
        story.state = story_selected;
        let mut boss_selected = story.boss_phase;
        egui::ComboBox::from_label("Boss State")
                .selected_text(format!("{:?}", boss_selected))
                .show_ui(ui, |ui| {
            for boss_state in BossPhase::iter() {
                ui.selectable_value(&mut boss_selected, boss_state, format!("{:?}", boss_state));
            }
        });
        story.boss_phase = boss_selected;
        ui.horizontal(|ui| {
            let mut researchlab_cleared = story.researchlab_cleared;
            ui.checkbox(&mut researchlab_cleared, "Research Lab Cleared");
            story.researchlab_cleared = researchlab_cleared;
        });
        ui.horizontal(|ui| {
            let mut military_cleared = story.military_cleared;
            ui.checkbox(&mut military_cleared, "Military Outpost Cleared");
            story.military_cleared = military_cleared;
        });
        ui.horizontal(|ui| {
            let mut monastry_cleared = story.monastry_cleared;
            ui.checkbox(&mut monastry_cleared, "Monastry Cleared");
            story.monastry_cleared = monastry_cleared;
        });
        ui.horizontal(|ui| {
            let mut luxury_cleared = story.luxury_cleared;
            ui.checkbox(&mut luxury_cleared, "Villa Cleared");
            story.luxury_cleared = luxury_cleared;
        });
        ui.horizontal(|ui| {
            let mut companion = story.companion;
            ui.checkbox(&mut companion, "Drone Companion");
            story.companion = companion;
        });
        ui.horizontal(|ui| {
            let mut surface = story.surface;
            ui.checkbox(&mut surface, "Surface unlocked");
            story.surface = surface;
        });
        ui.horizontal(|ui| {
            let mut review = story.review;
            ui.checkbox(&mut review, "End Review Available");
            story.review = review;
        });
    });
}

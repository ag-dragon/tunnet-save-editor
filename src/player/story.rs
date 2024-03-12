use crate::save_file::SaveFile;

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

    pub movement: bool,
    pub look: bool,
    pub sprint: bool,

    pub page_no: i32,
    pub pages: i32,

    pub inventory: crate::player::inventory::Inventory,
    pub knowledge: crate::player::journal::Knowledge,
    pub home: crate::player::home::Home,
    pub visited_chunks: Vec<crate::ChunkCoords>,
    pub map_annotations: crate::MapAnnotations,

    //connection_status: Vec
    // streaks?
    // mainframes: Vec?
}

pub fn story_editor(ui: &mut egui::Ui, save_file: &mut SaveFile) {
    ui.collapsing("Story Progress", |ui| {
        let mut story_selected = save_file.story.state;
        egui::ComboBox::from_label("Story State")
                .selected_text(format!("{:?}", story_selected))
                .show_ui(ui, |ui| {
            for story_state in StoryState::iter() {
                ui.selectable_value(&mut story_selected, story_state, format!("{:?}", story_state));
            }
        });
        save_file.story.state = story_selected;
        let mut boss_selected = save_file.story.boss_phase;
        egui::ComboBox::from_label("Boss State")
                .selected_text(format!("{:?}", boss_selected))
                .show_ui(ui, |ui| {
            for boss_state in BossPhase::iter() {
                ui.selectable_value(&mut boss_selected, boss_state, format!("{:?}", boss_state));
            }
        });
        save_file.story.boss_phase = boss_selected;
        ui.horizontal(|ui| {
            let mut researchlab_cleared = save_file.story.researchlab_cleared;
            ui.checkbox(&mut researchlab_cleared, "Research Lab Cleared");
            save_file.story.researchlab_cleared = researchlab_cleared;
        });
        ui.horizontal(|ui| {
            let mut military_cleared = save_file.story.military_cleared;
            ui.checkbox(&mut military_cleared, "Military Outpost Cleared");
            save_file.story.military_cleared = military_cleared;
        });
        ui.horizontal(|ui| {
            let mut monastry_cleared = save_file.story.monastry_cleared;
            ui.checkbox(&mut monastry_cleared, "Monastry Cleared");
            save_file.story.monastry_cleared = monastry_cleared;
        });
        ui.horizontal(|ui| {
            let mut luxury_cleared = save_file.story.luxury_cleared;
            ui.checkbox(&mut luxury_cleared, "Villa Cleared");
            save_file.story.luxury_cleared = luxury_cleared;
        });
        ui.horizontal(|ui| {
            let mut companion = save_file.story.companion;
            ui.checkbox(&mut companion, "Drone Companion");
            save_file.story.companion = companion;
        });
        ui.horizontal(|ui| {
            let mut surface = save_file.story.surface;
            ui.checkbox(&mut surface, "Surface unlocked");
            save_file.story.surface = surface;
        });
        ui.horizontal(|ui| {
            let mut review = save_file.story.review;
            ui.checkbox(&mut review, "End Review Available");
            save_file.story.review = review;
        });
    });
}

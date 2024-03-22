use tunnet_save::player::story::{Story, StoryState, BossPhase};

use bevy_egui::egui;

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

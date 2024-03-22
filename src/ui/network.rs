use bevy_egui::egui;

pub fn network_editor(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Network");
    });
}

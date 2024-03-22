use tunnet_save::player::guide::Guide;

use bevy_egui::egui;

pub fn guide_editor(ui: &mut egui::Ui, guide: &mut Guide) {
    ui.collapsing("Guidebook", |ui| {
        ui.horizontal(|ui| {
            ui.label("Current Page:");
            let mut page = guide.page_no;
            ui.add(egui::DragValue::new(&mut page).speed(1).clamp_range(1..=52));
            guide.page_no = page;
        });
        ui.horizontal(|ui| {
            ui.label("# of Pages:");
            let mut pages = guide.pages;
            ui.add(egui::DragValue::new(&mut pages).speed(1).clamp_range(1..=52));
            guide.pages = pages;
        });
    });
}

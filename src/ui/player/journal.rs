use tunnet_save::player::journal::{Knowledge, JournalEntry};

use bevy_egui::egui;

pub fn journal_editor(ui: &mut egui::Ui, knowledge: &mut Knowledge) {
    ui.collapsing("Journal", |ui| {
        ui.horizontal(|ui| {
            let mut unread = knowledge.unread;
            ui.checkbox(&mut unread, "Unread Symbol");
            knowledge.unread = unread;
        });
        ui.collapsing("Entries", |ui| {
            for entry in JournalEntry::iter() {
                ui.horizontal(|ui| {
                    let mut note = knowledge.journal.contains(&entry);
                    ui.checkbox(&mut note, format!("{:?}", entry));
                    if !note && knowledge.journal.contains(&entry) {
                        knowledge.journal.retain(|&x| x != entry);
                    } else if note && !knowledge.journal.contains(&entry) {
                        knowledge.journal.push(entry);
                    }
                });
            }
        });
    });
}

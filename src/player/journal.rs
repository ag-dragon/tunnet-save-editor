use bevy_egui::egui;
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Knowledge {
    pub unread: bool,
    pub journal: Vec<JournalEntry>,
}

#[derive(EnumIter, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum JournalEntry {
    Journal,
    LongHibernation,
    JobDescription,
    RelayAvailable,
    HubAvailable,
    Filter,
    FilterAvailable,
    Antivirus,
    AntivirusAvailable,
    Combo,
    ConnectionStatus,
    CannotReachSurface,
    RobotsHaveAFearModule,
    RobotsCannotSurviveAtTheSurface,
    OtherMainframesArePresent,
    Antenna,
    LongRangeScanner,
    Tester,
    OwnCabin,
    SupervisorWokeTechnicianUp,
    SupervisorWasHiredByTheArchitects,
    SupervisorName,
    SupervisorDoesNotRespond,
    SupervisorNeedToFind,
    SupervisorNeedsHelp,
    SupervisorRadio,
    SupervisorExploresDNASequence,
    ArchitectsExist,
    ArchitectsCreatedTheBunkers,
    ArchitectsCreatedTheRobots,
    ArchitectsDisappeared,
    ArchitectsDidNotFinish,
    ArchitectsFoundLifeForm,
    MadelineTrappedTheArchitects,
    ArchitectsDiedInHQ,
    Apocalypse,
    ApocalypseDidNotHappen,
    Infection,
    InfectionEffects,
    Stalker,
    Drone,
    DroneShowsDirection,
    DroneNeedsHelp,
    CookSellsBatteries,
    FishermanSellsScrap,
    FishermanHasAMagnet,
    FishermanLentMeHisMagnet,
    FarmerTradesVegetablesForSeeds,
    HermitLooksForADisk,
    HermitTradesOilForVegetables,
    GrandmaTradesToysForOil,
    GrandmaNetworkUsage,
    GrandpaNeedsABattery,
    GrandpaHasANewBattery,
    GrandpaWantsToHelpGrandson,
    KidWantsAToy,
    SoldiersGotTheMagnets,
    SoldiersNeedOil,
    OperatorNeedsAScrew,
    OperatorTradesSpeakersForMagnets,
    OperatorCanWork,
    GrandsonJoinedTheScout,
    ScoutTradesSeedsForScraps,
    DJTradesDisksForSpeakers,
    CashierBuysAnything,
    SpidersDoNotLikeLight,
    MonksShouldNotBeLookedAt,
    SirensShouldBeLookedAt,
    AbandonedLabQuiet,
    UndergroundComplex,
    ScoutsAreHiring,
    GrandsonAgreedToJoin,
    GrandsonExists,
    VIPCodePhrase,
    VIPCode,
    NetworkUsageScientist,
    MadelineIsMad,
    MadelineHasAnIdea,
    MadelineTookBattery,
    MadelineNeedsBattery,
    TechnicanNameDefault,
    TechnicianNameHarry,
    TechnicianNameRoger,
    TechnicianNameTherese,
    TechnicianNameChantal,
    DJNetworkUsage,
    DJHasASpeaker,
    ShelterNetworkUsage,
    BossCanDisinfect,
    Retire,
}

pub fn journal_editor(ui: &mut egui::Ui, save_file: &mut crate::SaveFile) {
    ui.collapsing("Journal", |ui| {
        ui.horizontal(|ui| {
            let mut unread = save_file.story.knowledge.unread;
            ui.checkbox(&mut unread, "Unread Symbol");
            save_file.story.knowledge.unread = unread;
        });
        ui.collapsing("Entries", |ui| {
            for entry in JournalEntry::iter() {
                ui.horizontal(|ui| {
                    let mut note = save_file.story.knowledge.journal.contains(&entry);
                    ui.checkbox(&mut note, format!("{:?}", entry));
                    if !note && save_file.story.knowledge.journal.contains(&entry) {
                        save_file.story.knowledge.journal.retain(|&x| x != entry);
                    } else if note && !save_file.story.knowledge.journal.contains(&entry) {
                        save_file.story.knowledge.journal.push(entry);
                    }
                });
            }
        });
    });
}

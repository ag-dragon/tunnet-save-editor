use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Knowledge {
    pub unread: bool,
    pub journal: Vec<JournalEntry>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
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

impl JournalEntry {
    pub fn iter() -> impl Iterator<Item = JournalEntry> {
        use JournalEntry::*;
        [
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
        ].iter().copied()
    }
}

use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{FontData, FontDefinitions, FontFamily};
use serde::{Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use std::{fs::File, io::BufReader, io::Write, collections::HashMap};

pub mod player;

#[derive(Resource, Default)]
enum EditorTab {
    #[default]
    Player,
    Network,
    Chunks,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Player {
    pos: [f64; 3],
    credits: i32,
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

#[derive(Serialize, Deserialize, Default, Debug)]
struct ChunkCoords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct MapAnnotations {
    annotations: Vec<[Annotation; 2]>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
enum MapColor {
    #[default]
    Pink,
    Yellow,
    Green,
    Blue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Annotation {
    Coords(ChunkCoords),
    Description { color: MapColor, note: String },
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Knowledge {
    unread: bool,
    journal: Vec<JournalEntry>,
}

#[derive(EnumIter, Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
enum JournalEntry {
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

#[derive(Serialize, Deserialize, Default, Debug)]
struct Home {
    items: Vec<Furniture>,
    ads: Vec<Ad>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
enum Furniture {
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
struct Ad {
    id: i64,
    title: String,
    price: i32,
    seller: Address,
    qty: i32,
    item: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Address {
    elements: Vec<BaseFour>,
    address_type: AddressType,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
enum AddressType {
    Endpoint,
    Filter,
    UnrestrictedFilter,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
enum BaseFour {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Story {
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

    shop_level: i32,
    digging: bool,
    relay: bool,
    hub: bool,
    filter: bool,
    scan_short: bool,
    scan_long: bool,
    jetpack: bool,
    antivirus: bool,
    optical_fiber: bool,
    antenna: bool,
    tester: bool,
    relay_light: bool,
    patch: bool, //?
    filter_collision: bool,
    filter_full_address: bool,
    tester_repeat: bool,
    tester_spoof: bool,
    tester_snoop: bool,
    scan_short_enhanced: bool,
    scan_long_peers: bool,
    auto_map: bool,

    movement: bool,
    look: bool,
    sprint: bool,

    page_no: i32,
    pages: i32,

    inventory: player::inventory::Inventory,
    knowledge: Knowledge,
    home: Home,
    visited_chunks: Vec<ChunkCoords>,
    map_annotations: MapAnnotations,

    //connection_status: Vec
    // streaks?
    // mainframes: Vec?
}

#[derive(Resource, Serialize, Deserialize, Default, Debug)]
struct SaveFile {
    player: Player,
    story: Story,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<EditorTab>()
        .init_resource::<SaveFile>()
        .add_systems(Startup, setup)
        .add_systems(Startup, editor_ui_setup)
        .add_systems(Update, editor_ui)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(40.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn editor_ui_setup(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    // Change font to tunnet font
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert("Flexi_IBM_VGA_True".to_owned(),
        FontData::from_static(include_bytes!("../assets/fonts/Flexi_IBM_VGA_True.ttf")));

    fonts.families.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "Flexi_IBM_VGA_True".to_owned());

    fonts.families.get_mut(&FontFamily::Monospace).unwrap()
        .push("Flexi_IBM_VGA_True".to_owned());

    ctx.set_fonts(fonts);

    // Change styling
}

fn editor_ui(mut contexts: EguiContexts, mut editor_tab: ResMut<EditorTab>, mut save_file: ResMut<SaveFile>) {
    let ctx = contexts.ctx_mut();
    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Tunnet Save Editor");

            ui.horizontal(|ui| {
                if ui.button("Load").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                            .add_filter("json", &["json"])
                            .add_filter("*", &["*"])
                            .set_directory("./example_saves")
                            .pick_file() {
                        let file = File::open(path.display().to_string()).unwrap();
                        let reader = BufReader::new(file);

                        let deserialize_result = serde_json::from_reader(reader);
                        match deserialize_result {
                            Ok(result) => *save_file = result,
                            Err(e) => {
                                println!("{:?}", e);
                            }
                        };
                        //*save_file = serde_json::from_reader(reader).unwrap();
                    }
                }
                if ui.button("Save").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                            .add_filter("json", &["json"])
                            .add_filter("*", &["*"])
                            .set_directory("./example_saves")
                            .set_file_name("save.json")
                            .save_file() {
                        let mut file = File::create(path.display().to_string()).unwrap();
                        file.write_all(serde_json::to_string(&*save_file).unwrap().as_bytes()).unwrap();
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Player").clicked() {
                    *editor_tab = EditorTab::Player;
                }
                if ui.button("Network").clicked() {
                    *editor_tab = EditorTab::Network;
                }
                if ui.button("Chunks").clicked() {
                    *editor_tab = EditorTab::Chunks;
                }
            });

            egui::ScrollArea::vertical().show(ui, |ui| {
                match *editor_tab {
                    EditorTab::Player => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Player");


                            ui.collapsing("Player Data", |ui| {
                                ui.collapsing("Player Position", |ui| {
                                    ui.horizontal(|ui| {
                                        let mut px = save_file.player.pos[0];
                                        ui.add(egui::DragValue::new(&mut px).speed(0.1).prefix("x: "));
                                        save_file.player.pos[0] = px;
                                        let mut py = save_file.player.pos[1];
                                        ui.add(egui::DragValue::new(&mut py).speed(0.1).prefix("y: "));
                                        save_file.player.pos[1] = py;
                                        let mut pz = save_file.player.pos[2];
                                        ui.add(egui::DragValue::new(&mut pz).speed(0.1).prefix("z: "));
                                        save_file.player.pos[2] = pz;
                                    });
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Credits:");
                                    let mut credits = save_file.player.credits;
                                    ui.add(egui::DragValue::new(&mut credits).speed(0.1));
                                    save_file.player.credits = credits;
                                });
                                ui.horizontal(|ui| {
                                    let mut movement = save_file.story.movement;
                                    ui.checkbox(&mut movement, "Can Walk/Sprint");
                                    save_file.story.movement = movement;
                                });
                                ui.horizontal(|ui| {
                                    let mut look = save_file.story.look;
                                    ui.checkbox(&mut look, "Can Move Camera");
                                    save_file.story.look = look;
                                });
                            });

                            ui.collapsing("Shop Unlocks", |ui| {
                                ui.horizontal(|ui| {
                                    let mut drill = save_file.story.digging;
                                    ui.checkbox(&mut drill, "Drill");
                                    save_file.story.digging = drill;
                                });
                                ui.horizontal(|ui| {
                                    let mut patch = save_file.story.patch;
                                    ui.checkbox(&mut patch, "Drill Patching");
                                    save_file.story.patch = patch;
                                });
                                ui.horizontal(|ui| {
                                    let mut relay = save_file.story.relay;
                                    ui.checkbox(&mut relay, "Relay");
                                    save_file.story.relay = relay;
                                });
                                ui.horizontal(|ui| {
                                    let mut relay_light = save_file.story.relay_light;
                                    ui.checkbox(&mut relay_light, "Relay Light");
                                    save_file.story.relay_light = relay_light;
                                });
                                ui.horizontal(|ui| {
                                    let mut hub = save_file.story.hub;
                                    ui.checkbox(&mut hub, "Hub");
                                    save_file.story.hub = hub;
                                });
                                ui.horizontal(|ui| {
                                    let mut filter = save_file.story.filter;
                                    ui.checkbox(&mut filter, "Filter");
                                    save_file.story.filter = filter;
                                });
                                ui.horizontal(|ui| {
                                    let mut filter_collision = save_file.story.filter_collision;
                                    ui.checkbox(&mut filter_collision, "Filter Collision Handling");
                                    save_file.story.filter_collision = filter_collision;
                                });
                                ui.horizontal(|ui| {
                                    let mut filter_full_address = save_file.story.filter_full_address;
                                    ui.checkbox(&mut filter_full_address, "Filter Full Address");
                                    save_file.story.filter_full_address = filter_full_address;
                                });
                                ui.horizontal(|ui| {
                                    let mut scan_short = save_file.story.scan_short;
                                    ui.checkbox(&mut scan_short, "Short-Range Rader");
                                    save_file.story.scan_short = scan_short;
                                });
                                ui.horizontal(|ui| {
                                    let mut scan_short_enhanced = save_file.story.scan_short_enhanced;
                                    ui.checkbox(&mut scan_short_enhanced, "Short-Range Rader Enhanced");
                                    save_file.story.scan_short_enhanced = scan_short_enhanced;
                                });
                                ui.horizontal(|ui| {
                                    let mut scan_long = save_file.story.scan_long;
                                    ui.checkbox(&mut scan_long, "Long-Range Radar");
                                    save_file.story.scan_long = scan_long;
                                });
                                ui.horizontal(|ui| {
                                    let mut scan_long_peers = save_file.story.scan_long_peers;
                                    ui.checkbox(&mut scan_long_peers, "Long-Range Radar Peers");
                                    save_file.story.scan_long_peers = scan_long_peers;
                                });
                                ui.horizontal(|ui| {
                                    let mut antivirus = save_file.story.antivirus;
                                    ui.checkbox(&mut antivirus, "Antivirus");
                                    save_file.story.antivirus = antivirus;
                                });
                                ui.horizontal(|ui| {
                                    let mut tester = save_file.story.tester;
                                    ui.checkbox(&mut tester, "Tester");
                                    save_file.story.tester = tester;
                                });
                                ui.horizontal(|ui| {
                                    let mut tester_repeat = save_file.story.tester_repeat;
                                    ui.checkbox(&mut tester_repeat, "Tester Repeat");
                                    save_file.story.tester_repeat = tester_repeat;
                                });
                                ui.horizontal(|ui| {
                                    let mut tester_spoof = save_file.story.tester_spoof;
                                    ui.checkbox(&mut tester_spoof, "Tester Spoof");
                                    save_file.story.tester_spoof = tester_spoof;
                                });
                                ui.horizontal(|ui| {
                                    let mut tester_snoop = save_file.story.tester_snoop;
                                    ui.checkbox(&mut tester_snoop, "Tester Snoop");
                                    save_file.story.tester_snoop = tester_snoop;
                                });
                                ui.horizontal(|ui| {
                                    let mut jetpack = save_file.story.jetpack;
                                    ui.checkbox(&mut jetpack, "Jetpack");
                                    save_file.story.jetpack = jetpack;
                                });
                                ui.horizontal(|ui| {
                                    let mut auto_map = save_file.story.auto_map;
                                    ui.checkbox(&mut auto_map, "Auto-Map");
                                    save_file.story.auto_map = auto_map;
                                });
                            });

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

                            ui.collapsing("Inventory", |ui| {
                                for item in player::inventory::InventoryItem::iter() {
                                    let mut item_count: i32 = 0;
                                    match save_file.story.inventory.items.get(&format!("{:?}", item).to_lowercase()) {
                                        Some(count) => {
                                            item_count = *count;
                                        },
                                        _ => {},
                                    }
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{:?}", item));
                                        ui.add(egui::DragValue::new(&mut item_count).clamp_range(0..=99).speed(1));
                                    });
                                    if item_count > 0 {
                                        save_file.story.inventory.items.insert(format!("{:?}", item).to_lowercase(), item_count);
                                    } else if item_count == 0 {
                                        save_file.story.inventory.items.remove(&format!("{:?}", item).to_lowercase());
                                    }
                                }
                            });

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

                            ui.collapsing("Guidebook", |ui| {
                                // page_no
                                // pages
                            });
                        });
                    },
                    EditorTab::Network => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Network");
                        });
                    },
                    EditorTab::Chunks => {
                        ui.vertical_centered(|ui| {
                            ui.heading("Chunks");
                        });
                    },
                }
            });
        });
}

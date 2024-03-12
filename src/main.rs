pub mod save_file;
pub mod player;
pub mod network;

use save_file::SaveFile;

use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{FontData, FontDefinitions, FontFamily};
use serde::{Serialize, Deserialize};
use std::{fs::File, io::BufReader, io::Write};

#[derive(Resource, Default)]
enum EditorTab {
    #[default]
    Player,
    Network,
    Chunks,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChunkCoords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MapAnnotations {
    annotations: Vec<[Annotation; 2]>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub enum MapColor {
    #[default]
    Pink,
    Yellow,
    Green,
    Blue,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Annotation {
    Coords(ChunkCoords),
    Description { color: MapColor, note: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    elements: Vec<BaseFour>,
    address_type: AddressType,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum AddressType {
    Endpoint,
    Filter,
    UnrestrictedFilter,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Debug)]
pub enum BaseFour {
    Zero,
    One,
    Two,
    Three,
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
                        player::player_editor(ui, &mut save_file);
                    },
                    EditorTab::Network => {
                        network::network_editor(ui);
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

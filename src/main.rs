mod save_file;
mod player;
mod network;
mod chunks;
mod chunk_renderer;

use save_file::SaveFile;
use chunk_renderer::{GenBlockMeshEvent};
use chunks::ChunkCoords;

use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use egui::{FontData, FontDefinitions, FontFamily};
use std::{fs::File, io::BufReader, io::Write};

#[derive(Resource, Default)]
enum EditorTab {
    #[default]
    Player,
    Network,
    Chunks,
}

#[derive(Resource, Default)]
struct CurrentChunk(ChunkCoords);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugins(EguiPlugin)
        .add_plugins(PanOrbitCameraPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<EditorTab>()
        .init_resource::<SaveFile>()
        .init_resource::<CurrentChunk>()
        .add_event::<chunk_renderer::GenBlockMeshEvent>()
        .add_systems(Startup, chunk_renderer::chunk_setup)
        .add_systems(Startup, editor_ui_setup)
        .add_systems(Update, (editor_ui, chunk_renderer::update_chunk))
        .run();
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

fn editor_ui(
    mut contexts: EguiContexts,
    mut editor_tab: ResMut<EditorTab>,
    mut save_file: ResMut<SaveFile>,
    mut current_chunk: ResMut<CurrentChunk>,
    mut ev_genblockmesh: EventWriter<GenBlockMeshEvent>,
) {
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

                        current_chunk.0.x = save_file.player.pos[0] as i32 / 16;
                        current_chunk.0.y = save_file.player.pos[1] as i32 / 16;
                        current_chunk.0.z = save_file.player.pos[2] as i32 / 16;
                        ev_genblockmesh.send(GenBlockMeshEvent);
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
                        chunks::chunk_editor(ui, ev_genblockmesh, current_chunk);
                    },
                }
            });
        });
}

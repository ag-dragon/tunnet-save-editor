mod ui;
mod chunk_renderer;

use tunnet_save::{
    save_file::SaveFile,
    chunks::ChunkCoords,
};
use ui::EditorUiPlugin;
use chunk_renderer::ChunkRendererPlugin;

use bevy::{
    prelude::*,
    winit::WinitSettings,
};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

#[derive(Resource, Default)]
struct CurrentChunk(ChunkCoords);

#[derive(Resource, Default)]
struct CurrentSave(SaveFile);

fn main() {
    App::new()
        .init_resource::<CurrentSave>()
        .init_resource::<CurrentChunk>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WinitSettings::desktop_app())
        .add_event::<chunk_renderer::GenBlockMeshEvent>()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugins((
            EditorUiPlugin,
            ChunkRendererPlugin,
            PanOrbitCameraPlugin,
        ))
        .run();
}

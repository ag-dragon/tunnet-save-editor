mod ui;
mod chunk_editor;

use tunnet_save::{
    save_file::SaveFile,
    chunks::ChunkCoords,
};
use ui::EditorUiPlugin;
use chunk_editor::{ChunkEditorPlugin, GenBlockMeshEvent};

use bevy::prelude::*;

#[derive(Resource, Default)]
struct CurrentChunk(ChunkCoords);

#[derive(Resource, Default)]
struct CurrentSave(SaveFile);

fn main() {
    App::new()
        .init_resource::<CurrentSave>()
        .init_resource::<CurrentChunk>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_event::<GenBlockMeshEvent>()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest(),
        ))
        .add_plugins((
            EditorUiPlugin,
            ChunkEditorPlugin,
        ))
        .run();
}

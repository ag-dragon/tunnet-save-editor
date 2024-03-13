use bevy::prelude::*;
use bevy_egui::egui;
use bevy_panorbit_camera::PanOrbitCamera;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChunkCoords {
    x: i32,
    y: i32,
    z: i32,
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MapAnnotations {
    annotations: Vec<[Annotation; 2]>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Map {
    visited_chunks: Vec<ChunkCoords>,
    map_annotations: MapAnnotations,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChunkRoom {
    CloisterCeiling,
    SwimmingPool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ChunkType {
    Coords(ChunkCoords),
    Rooms {
        #[serde(rename = "BunkerRoom")]
        bunker_room: [Option<ChunkRoom>; 3],
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Chunk {
    Coords(ChunkCoords),
    Data(Vec<[i32; 2]>),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChunkData {
    chunk_types: Vec<Vec<ChunkType>>,
    chunks: Vec<Vec<Chunk>>,
}

#[derive(Component)]
pub enum VoxelType {
    Air,
    Dirt,
    UnbreakableDirt,
    Grass,
    UnbreakableGrass,
    UnbreakableRock,
    MetalWall,
    Tiles,
    MetalSheets,
    GrayBrick,
    Terraforming,
    UnbreakableCobble,
    RedGlyph,
    FlowerGlyph,
    WoodPlanks,
    Tiles2,
    CorruptedMetalWall,
    Air2,
    Magma,
    Air3,
    WobblyAir1,
    WobblyAir2,
    Air4,
    Watefall1,
    Watefall2,
    Watefall3,
    Watefall4,
    Watefall5,
    Watefall6,
    Watefall7,
    Watefall8,
    Watefall9,
    Watefall10,
    Watefall11,
    Watefall12,
    Watefall13,
    Watefall14,
    Watefall15,
    Watefall16,
    UnbreakableRock2,
    Air5,
    Air6,
    Air7,
    Air8,
    DamagedRock,
}

pub fn chunk_editor(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Chunks");
    });
}

pub fn chunk_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 400.0,
    });

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(32.0, 64.0, 64.0).looking_at(Vec3::new(16.5, 16.5, 16.5), Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));

    let mesh = meshes.add(Cuboid::new(0.1, 0.1, 0.1));
    for z in 0..32 {
        for y in 0..32 {
            for x in 0..32 {
                commands.spawn((PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(Color::WHITE),
                    transform: Transform::from_xyz(x as f32, y as f32, z as f32),
                    visibility: Visibility::Hidden,
                    ..default()
                },VoxelType::Air));
            }
        }
    }
}

pub fn draw_chunk(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Visibility, &mut VoxelType)>
) {
    for (tranform, mut visibility, voxel_type) in query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

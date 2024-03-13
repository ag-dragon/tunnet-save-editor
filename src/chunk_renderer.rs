use crate::chunks::voxels::VoxelType;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;

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

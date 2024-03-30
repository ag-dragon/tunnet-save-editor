use crate::{CurrentSave, GenBlockMeshEvent};
use crate::chunk_editor::{ChunkShape, CurrentChunk};

use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
};
use bevy_mod_raycast::prelude::*;
use block_mesh::ndshape::ConstShape;

#[derive(Resource)]
pub struct CameraSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.5,
            speed: 20.0,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_systems(Startup, camera_setup)
            .add_systems(Update, (
                camera_move,
                camera_look,
                camera_edit,
            ));
    }
}


fn camera_setup(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(32.0, 64.0, 64.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn camera_move(
    settings: Res<CameraSettings>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut camera_transform = query.get_single_mut().expect("Error getting camera entity");

    let local_x = camera_transform.local_x();
    let local_y = camera_transform.local_y();
    let local_z = camera_transform.local_z();

    let mut velocity = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        velocity -= Vec3::from(local_z);
    } else if keys.pressed(KeyCode::KeyS) {
        velocity += Vec3::from(local_z);
    }

    if keys.pressed(KeyCode::KeyD) {
        velocity += Vec3::from(local_x);
    } else if keys.pressed(KeyCode::KeyA) {
        velocity -= Vec3::from(local_x);
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        velocity += Vec3::from(local_y);
    } else if keys.pressed(KeyCode::ControlLeft) {
        velocity -= Vec3::from(local_y);
    }

    camera_transform.translation += velocity * settings.speed * time.delta_seconds();
}

fn camera_look(
    settings: Res<CameraSettings>,
    mut query: Query<&mut Transform, With<Camera>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
) {
    let mut camera_transform = query.get_single_mut().expect("Error getting camera entity");

    if mouse_buttons.pressed(MouseButton::Right) {
        for ev in motion.read() {
            let (mut yaw, mut pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);

            pitch -= (ev.delta.y * settings.sensitivity).to_radians();
            yaw -= (ev.delta.x * settings.sensitivity).to_radians();

            pitch = pitch.clamp(-1.54, 1.54);

            camera_transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn camera_edit(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    mut query: Query<&Transform, With<Camera>>,
    mut save_file: ResMut<CurrentSave>,
    current_chunk: Res<CurrentChunk>,
    mut ev_genblockmesh: EventWriter<GenBlockMeshEvent>,
) {
    let camera_transform = query.get_single_mut().expect("error getting camera entity");

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let pos = camera_transform.translation;
        let dir = -camera_transform.local_z();
        let intersections = raycast.debug_cast_ray(Ray3d::new(pos, Vec3::from(dir)), &default(), &mut gizmos);
        if intersections.len() > 0 {
            let intersection = &intersections[0].1;
            let position = intersection.position() - (intersection.normal() * 0.1);
            gizmos.cuboid(
                Transform::from_xyz(position.x.floor()+0.5, position.y.floor()+0.5, position.z.floor()+0.5),
                Color::RED,
            );

            match save_file.0.chunk_data.chunks.get_mut(&current_chunk.0) {
                Some(rle_chunk) => {
                    let mut voxel_data = super::decode_rle(rle_chunk);

                    
                    voxel_data[ChunkShape::linearize([
                        (position.x - 1.0).floor() as u32,
                        (position.y - 1.0).floor() as u32,
                        (position.z - 1.0).floor() as u32,
                    ]) as usize] = 0;

                    *rle_chunk = super::encode_rle(&voxel_data);
                    ev_genblockmesh.send(GenBlockMeshEvent);
                },
                None => {},
            }
        }
    }
}

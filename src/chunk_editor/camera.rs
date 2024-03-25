use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, camera_update);
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

fn camera_update(
    mut query: Query<&mut Transform, With<Camera>>,
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

    camera_transform.translation += velocity;
}

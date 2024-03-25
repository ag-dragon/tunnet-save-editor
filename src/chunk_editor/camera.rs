use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
};

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

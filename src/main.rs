use bevy::{
    prelude::*,
    winit::WinitSettings,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Startup, ui)
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

fn ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // screen
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Start,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        // UI Box
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            border_color: Color::hex("454545FF").unwrap().into(),
            background_color: Color::hex("303030EA").unwrap().into(),
            ..default()
        }).with_children(|parent| {
            // Load/Save buttons
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                border_color: Color::hex("454545FF").unwrap().into(),
                ..default()
            }).with_children(|parent| {
                // Load button
                parent.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::hex("7F7F7FFF").unwrap().into(),
                    background_color: Color::hex("#5a5a5aFF").unwrap().into(),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Load",
                        TextStyle {
                            font: asset_server.load("fonts/Flexi_IBM_VGA_True.ttf"),
                            font_size: 16.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
                // Save button
                parent.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::hex("7F7F7FFF").unwrap().into(),
                    background_color: Color::hex("#5a5a5aFF").unwrap().into(),
                    ..default()
                }).with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Save",
                        TextStyle {
                            font: asset_server.load("fonts/Flexi_IBM_VGA_True.ttf"),
                            font_size: 16.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            });
            // Section Tabs
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    border: UiRect::bottom(Val::Px(2.0)),
                    ..default()
                },
                border_color: Color::hex("454545FF").unwrap().into(),
                ..default()
            });
        });
    });
}

mod general;
mod obstacle;
mod player;

use bevy::{
    app::AppExit,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use general::constants::PIXELS_PER_METER;
use obstacle::spawn_obstacle;
use player::{player_movement, spawn_player};

// Marker component for the game's camera
#[derive(Component)]
struct GameCamera;

// Marker component for the FPS text
#[derive(Component)]
struct FPSText {
    timer: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "The Caverns".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_startup_system(spawn_player)
        .add_system(fps_text_system)
        .add_system(handle_input)
        .add_system(player_movement.after(handle_input))
        .run();
}

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2dBundle::default(), GameCamera));

    commands.spawn((
        TextBundle::from_section(
            "FPS: -1",
            TextStyle {
                font: asset_server.load("fonts/joystix monospace.otf"),
                font_size: 25.0,
                color: Color::GREEN,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        FPSText { timer: 0.0 },
    ));

    spawn_obstacle(
        &mut commands,
        Color::rgb(0.0, 0.0, 0.0),
        Some(Vec2::new(100.0, 100.0)),
        Transform::from_translation(Vec3::new(-150.0, 0.0, 0.0)),
    );

    spawn_obstacle(
        &mut commands,
        Color::rgb(0.5, 0.7, 0.0),
        Some(Vec2::new(50.0, 75.0)),
        Transform::from_translation(Vec3::new(200.0, 150.0, 0.0)),
    );
}

fn setup_physics(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(50.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(-150.0, 0.0, 0.0)));
}

fn handle_input(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        println!("Quitting...");
        exit.send(AppExit);
    }
}

fn fps_text_system(
    diagnostics: Res<Diagnostics>,
    time: Res<Time>,
    mut labels: Query<(&mut Text, &mut FPSText)>,
) {
    for mut text in &mut labels {
        text.1.timer += time.delta_seconds();
        if text.1.timer < 0.5 {
            continue;
        } else {
            text.1.timer = 0.0;
        }

        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.0.sections[0].value = format!("FPS: {value:.2}");
            }
        }
    }
}

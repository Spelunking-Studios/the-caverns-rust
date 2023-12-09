//! The Caverns - An ARPG Game With Fantasy and D&D Elements

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

mod constants;
mod data_store;
mod general;
mod map;
mod menu;
mod player;
mod util;

use bevy::{
    app::AppExit,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use constants::{LOG_FILTER, PIXELS_PER_METER};
use map::plugin::MapPlugin;
use menu::plugin::MenuPlugin;
use player::player_movement;

/// Marker component for the game's camera
#[derive(Component)]
struct GameCamera;

/// Marker component for the FPS text
#[derive(Component)]
struct FPSText {
    /// Timer for how long since the FPS UI was updated
    timer: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Caverns".into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: LOG_FILTER.into(),
                    level: bevy::log::Level::DEBUG,
                }),
        )
        .add_plugins((
            FrameTimeDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PIXELS_PER_METER),
            RapierDebugRenderPlugin::default(),
            MapPlugin::default(),
            MenuPlugin,
        ))
        .add_systems(Startup, (setup, create_fps_text))
        .add_systems(
            Update,
            (
                fps_text_system,
                handle_input,
                player_movement.after(handle_input),
            ),
        )
        .run();
}

/// Setup system that loads everything needed to get the game off the ground
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), GameCamera));
    debug!("Setup camera");
}

/// Spawns in the UI for the FPS counter
fn create_fps_text(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        FPSText { timer: 0.0 },
    ));
}

/// Handles generic input for the game
fn handle_input(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    // Quit
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

/// A system to display the game's current FPS
fn fps_text_system(
    diagnostics: Res<DiagnosticsStore>,
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn setup_worked() {
        // Setup app
        let mut app: App = App::new();

        // Add systems & plugins
        app.add_systems(Startup, setup);

        // Update the app
        app.update();

        // Validate world
        assert_eq!(app.world.query::<&GameCamera>().iter(&app.world).len(), 1);
    }
}

mod constants;
mod general;
mod map;
mod menu;
mod obstacle;
mod player;

use bevy::{
    app::AppExit,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    log::LogPlugin,
    prelude::*,
};
use bevy_rapier2d::prelude::*;

use constants::{LOG_FILTER, PIXELS_PER_METER};
use map::{
    plugin::MapPlugin,
    state::{MapReadinessState, MapState},
};
use menu::plugin::MenuPlugin;
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
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(MapPlugin::default())
        .add_plugin(MenuPlugin)
        .add_startup_system(setup)
        // .add_startup_system(create_fps_text)
        // .add_startup_system(spawn_obstacles)
        // .add_startup_system(spawn_player)
        .add_system(fps_text_system)
        .add_system(handle_input)
        .add_system(player_movement.after(handle_input))
        .run();
}

/// Setup system that loads everything needed to get the game off the ground
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn((Camera2dBundle::default(), GameCamera));
    debug!("Setup camera");
}

fn spawn_obstacles(mut commands: Commands) {
    /* spawn_obstacle(
        &mut commands,
        Color::rgb(0.1, 0.1, 0.1),
        Some(Vec2::new(64.0, 64.0)),
        Transform::from_translation(Vec3::new(-150.0, 0.0, 0.0)),
    );

    spawn_obstacle(
        &mut commands,
        Color::rgb(0.5, 0.7, 0.0),
        Some(Vec2::new(32.0, 48.0)),
        Transform::from_translation(Vec3::new(200.0, 150.0, 0.0)),
    ); */
}

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
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
            ..default()
        }),
        FPSText { timer: 0.0 },
    ));
}

fn handle_input(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        info!("Quitting...");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn setup_worked() {
        // Setup app
        let mut app: App = App::new();

        // Add systems & plugins
        app.add_startup_system(setup);

        // Update the app
        app.update();

        // Validate world
        assert_eq!(app.world.query::<&GameCamera>().iter(&app.world).len(), 1);
    }
}

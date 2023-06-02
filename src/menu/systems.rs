//! Defines all of the systems used by the menu plugin

use super::{state::GameMenuState, util::start_game};
use crate::map::state::{MapReadinessState, MapState};
use bevy::{
    app::AppExit,
    prelude::*
};

// Really helpful consts and macros
const YELLOW: Color = Color::hsl(50.69, 0.9684, 0.5039);
const BUTTON_COLOR: Color = YELLOW;
const BUTTON_COLOR_HOVER: Color = Color::hsl(50.69, 0.9684, 0.45);
const BUTTON_FONT: &str = "fonts/joystix monospace.otf";
const BUTTON_FONT_SIZE: f32 = 30.0;

/// A marker component for the start button
#[derive(Debug, Component)]
pub struct StartButton;

/// A marker component for the quit button
#[derive(Debug, Component)]
pub struct QuitButton;

/// Marks a UI node as being a root node of the UI
///
/// This is used to select all of the root nodes in the menu UI when the menu
/// is being despawned
#[derive(Debug, Component)]
pub struct MenuRootNode;

/// Spawns in the UI and is responsible for init of anything the menu needs
pub fn setup_start_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Setting up start screen");

    // Create a basic UI

    // Title
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect {
                    top: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(MenuRootNode)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "The Caverns",
                TextStyle {
                    font: asset_server.load(BUTTON_FONT),
                    font_size: 80.0,
                    color: YELLOW,
                },
            ));
        });

    // Buttons
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                gap: Size::new(Val::Px(0.0), Val::Percent(5.0)),
                ..default()
            },
            ..default()
        })
        .insert(MenuRootNode)
        .with_children(|parent| {
            // Start button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(StartButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start",
                        TextStyle {
                            font: asset_server.load(BUTTON_FONT),
                            font_size: BUTTON_FONT_SIZE,
                            color: Color::BLACK,
                        },
                    ));
                });

            // Quit button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(QuitButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load("fonts/joystix monospace.otf"),
                            font_size: BUTTON_FONT_SIZE,
                            color: Color::BLACK,
                        },
                    ));
                });
        });

    debug!("Start screen setup is complete");
}

/// Despawns the UI and is responsible for deinitializing anything the menu used
pub fn cleanup_start_screen(
    mut commands: Commands,
    menu_root_query: Query<Entity, With<MenuRootNode>>,
) {
    debug!("Cleaning up start screen");

    // Desawn the UI
    menu_root_query.for_each(|node| {
        commands.entity(node).despawn_recursive();
    });
}

/// Adds hover effects to buttons
#[allow(clippy::type_complexity)]
pub fn update_button_hover_state(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *color = BUTTON_COLOR.into();
            }
            _ => {}
        }
    }
}

/// Adds specific functionality to the start button
///
/// The system will detect when the start button has been clicked and will do 3
/// things:
/// 1. Switch the game's menu state to [InGame](GameMenuState::InGame)
/// 2. Starts loading in the map by setting the readiness state to
///    [Loading](MapReadinessState::Loading)
/// 3. Calls the [start_game](start_game) utility function to actually start the game.
pub fn update_start_button(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut map_state: ResMut<MapState>,
    mut next_map_state: ResMut<NextState<MapReadinessState>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    asset_server: Res<AssetServer>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            // Set the menu's state
            next_state.set(GameMenuState::InGame);

            // Start loading the map
            next_map_state.set(MapReadinessState::Loading);

            // Start the game
            start_game(&mut commands, &mut map_state, &asset_server);
        }
    }
}

/// Adds functionality to the quit button
pub fn update_quit_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut exit_event: EventWriter<AppExit>
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            exit_event.send(AppExit);
        }
    }
}

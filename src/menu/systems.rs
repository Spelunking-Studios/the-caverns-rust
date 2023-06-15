//! Defines all of the systems used by the menu plugin

use super::state::GameMenuState;
use crate::{
    constants::STORY_INTRO,
    map::state::{MapReadinessState, MapState},
    util::start_game,
};
use bevy::{
    app::AppExit,
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
    window::WindowResized,
};

// Really helpful consts and macros
/// The yellow shade used
const YELLOW: Color = Color::hsl(50.69, 0.9684, 0.5039);
/// Uhhhhh, look at the name of the constant
const BUTTON_COLOR: Color = YELLOW;
/// Uhhhhh, look at the name of the constant
const BUTTON_COLOR_HOVER: Color = Color::hsl(50.69, 0.9684, 0.45);
/// Uhhhhh, look at the name of the constant
const BUTTON_FONT: &str = "fonts/joystix monospace.otf";
/// Uhhhhh, look at the name of the constant
const TEXT_FONT: &str = "fonts/joystix monospace.otf";
/// Uhhhhh, look at the name of the constant
const BUTTON_FONT_SIZE: f32 = 30.0;
/// Percentage of the screen that the storyline intro would ideally occupy
const STORYLINE_INTRO_WPERCENT: f32 = 0.8;
/// Minimum size of the storyline intro text bounding box
const STORYLINE_INTRO_IDEAL_MIN: [f32; 2] = [1000.0, 500.0];

/// A marker component for the start button
#[derive(Debug, Component)]
pub struct StartButton;

/// A marker component for the quit button
#[derive(Debug, Component)]
pub struct QuitButton;

/// A marker component for the continue button
#[derive(Debug, Component)]
pub struct ContinueButton;

/// A marker component for the storyline intro text
#[derive(Debug, Component)]
pub struct StorylineIntroText;

/// A marker component for the storyline intro text sprite container
#[derive(Debug, Component)]
pub struct StorylineIntroSprite;

/// Marks a UI node as being a root node of the UI
///
/// This is used to select all of the root nodes in the menu UI when the menu
/// is being despawned
#[derive(Debug, Component)]
pub struct MenuRootNode;

/// [TODO] Answers the question it's named after
///
/// It will check the game's data store and determine if the storyline intro
/// has been shown before. The result of this determination will be returned.
pub fn storyline_intro_shown() -> bool {
    false
}

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
                    font: asset_server.load(TEXT_FONT),
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
                            font: asset_server.load(BUTTON_FONT),
                            font_size: BUTTON_FONT_SIZE,
                            color: Color::BLACK,
                        },
                    ));
                });
        });

    debug!("Start screen setup is complete");
}

/// Spawns in the UI for the storyline intro screen
pub fn setup_storyline_intro_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    debug!("Setting up the storyline intro screen");

    let window = windows.get_single_mut().unwrap();
    let window_size: [f32; 2] = [window.width(), window.height()];
    let font = asset_server.load(TEXT_FONT);
    // The bounds are calculated as:
    // The larger of either the 80% of the window dimension (width or height) or
    // 1000 (500 for height).
    // Then decide to take the smallest item: either the window's size (minus padding)
    // or the ideal box size. This ensures that the box is never bigger than the window.
    let bounds = Vec2::new(
        ((window_size[0] * STORYLINE_INTRO_WPERCENT).max(STORYLINE_INTRO_IDEAL_MIN[0]))
            .min(window_size[0] - 50.0),
        ((window_size[1] * STORYLINE_INTRO_WPERCENT).max(STORYLINE_INTRO_IDEAL_MIN[1]))
            .min(window_size[1] - 50.0),
    );

    // The parent must be a SpriteBundle for us to use Text2dBundle
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(bounds),
                ..default()
            },
            transform: Transform::from_translation(Vec3::splat(0.0)),
            ..default()
        })
        .insert((StorylineIntroSprite, MenuRootNode))
        .with_children(|parent| {
            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            STORY_INTRO.join("\n\n"),
                            TextStyle {
                                font,
                                font_size: 20.0 + (window_size[1] * 0.005),
                                color: YELLOW,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        linebreak_behaviour: BreakLineOn::WordBoundary,
                    },
                    text_2d_bounds: Text2dBounds { size: bounds },
                    // Ensure the text is drawn on top of the box
                    transform: Transform::from_translation(Vec3::Z),
                    ..default()
                })
                .insert(StorylineIntroText);
        });

    // Buttons require a NodeBundle parent
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::End,
                flex_direction: FlexDirection::Column,
                gap: Size::new(Val::Px(0.0), Val::Percent(5.0)),
                padding: UiRect {
                    bottom: Val::Px(25.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(MenuRootNode)
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BUTTON_COLOR.into(),
                    ..default()
                })
                .insert(ContinueButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font: asset_server.load(BUTTON_FONT),
                            font_size: BUTTON_FONT_SIZE,
                            color: Color::BLACK,
                        },
                    ));
                });
        });

    debug!("Storyline intro screen setup is complete");
}

/// Updates the storyline intro screen text
pub fn update_storyline_intro_screen_text(
    mut sprite_query: Query<&mut Sprite, With<StorylineIntroSprite>>,
    mut text_query: Query<&mut Text, With<StorylineIntroText>>,
    mut text_bounds_query: Query<&mut Text2dBounds, With<StorylineIntroText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.iter() {
        // Update the sprite size
        for mut sprite in &mut sprite_query {
            sprite.custom_size = Some(Vec2::new(
                (e.width * STORYLINE_INTRO_WPERCENT).max(STORYLINE_INTRO_IDEAL_MIN[0]),
                (e.height * STORYLINE_INTRO_WPERCENT).max(STORYLINE_INTRO_IDEAL_MIN[1]),
            ));
        }

        // Update the font size
        for mut text in &mut text_query {
            for section in &mut text.sections {
                section.style.font_size = 20.0 + (e.height * 0.005);
            }
        }

        // Update bounds
        for mut bounds in &mut text_bounds_query {
            bounds.size = Vec2::new(
                ((e.width * STORYLINE_INTRO_WPERCENT).max(STORYLINE_INTRO_IDEAL_MIN[0]))
                    .min(e.width - 50.0),
                ((e.height * STORYLINE_INTRO_WPERCENT).max(STORYLINE_INTRO_IDEAL_MIN[1]))
                    .min(e.height - 50.0),
            )
        }
    }
}

/// Adds specific functionality to the storyline itnro's continue button
///
/// The system will detect when the start button has been clicked and will do
/// the following:
/// 1. Update the data store to mark the storyline intro as having been seen.
/// 2. If so, switch to the [InGame](GameMenuState::InGame) state.
/// 3. If switching to the [StorylineIntro](GameMenuState::StorylineIntro) state, stop here.
/// 2. Starts loading in the map by setting the readiness state to
///    [Loading](MapReadinessState::Loading)
/// 3. Calls the [start_game](start_game) utility function to actually start the game.
pub fn update_storyline_intro_screen_btn(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    mut next_state: ResMut<NextState<GameMenuState>>,
    mut next_map_state: ResMut<NextState<MapReadinessState>>,
    asset_server: Res<AssetServer>,
    interaction_query: Query<&Interaction, (With<ContinueButton>, Changed<Interaction>)>,
) {
    for interaction in &interaction_query {
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

/// Despawns the UI and is responsible for deinitializing anything the menu used
pub fn cleanup_start_screen(mut commands: Commands, query: Query<Entity, With<MenuRootNode>>) {
    debug!("Cleaning up start screen");

    // Desawn the UI
    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
    });
}

/// Despawns the UI for the storyline intro screen
pub fn cleanup_storyline_intro_screen(
    mut commands: Commands,
    query: Query<Entity, With<MenuRootNode>>,
) {
    debug!("Cleaning up the storyline intro screen");

    query.for_each(|entity| {
        commands.entity(entity).despawn_recursive();
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
/// The system will detect when the start button has been clicked and will do
/// the following:
/// 1. Determine if the storyline intro needs to be shown
/// 2. If so, switch to the [StorylineIntro](GameMenuState::StorylineIntro) state
/// , otherwise, switch to [InGame](GameMenuState::InGame).
/// 3. If switching to the [StorylineIntro](GameMenuState::StorylineIntro) state, stop here.
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
            if !storyline_intro_shown() {
                next_state.set(GameMenuState::StorylineIntro);
                continue;
            }

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
    mut exit_event: EventWriter<AppExit>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Clicked {
            exit_event.send(AppExit);
        }
    }
}

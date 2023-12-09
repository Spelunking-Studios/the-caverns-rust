//! Plugin for the menu module which keeps all of the implementation details
//! within the menu module.
//!
//! This means that the the main function doesn't need to know all of the systems
//! nor has to add them.

use super::{
    state::GameMenuState,
    systems::{
        cleanup_start_screen, cleanup_storyline_intro_screen, setup_start_screen,
        setup_storyline_intro_screen, update_button_hover_state, update_quit_button,
        update_start_button, update_storyline_intro_screen_btn, update_storyline_intro_screen_text,
    },
};
use bevy::prelude::*;

/// Struct defining the menu plugin
#[derive(Debug, Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Add the basic game state
        app.add_state::<GameMenuState>();

        // Add the setup system
        app.add_systems(OnEnter(GameMenuState::StartScreen), setup_start_screen);
        app.add_systems(
            OnEnter(GameMenuState::StorylineIntro),
            setup_storyline_intro_screen,
        );

        // Add the cleanup system
        app.add_systems(OnExit(GameMenuState::StartScreen), cleanup_start_screen);
        app.add_systems(
            OnExit(GameMenuState::StorylineIntro),
            cleanup_storyline_intro_screen,
        );

        // Add the various update systems
        app.add_systems(
            Update,
            (
                update_button_hover_state,
                update_start_button,
                update_quit_button,
            )
                .run_if(in_state(GameMenuState::StartScreen)),
        );

        app.add_systems(
            Update,
            (
                update_storyline_intro_screen_text,
                update_storyline_intro_screen_btn,
                update_button_hover_state,
            )
                .run_if(in_state(GameMenuState::StorylineIntro)),
        );
        debug!("MenuPlugin loaded");
    }
}

use super::{
    state::GameMenuState,
    systems::{
        cleanup_start_screen, cleanup_storyline_intro_screen, setup_start_screen,
        setup_storyline_intro_screen, update_button_hover_state, update_quit_button,
        update_start_button, update_storyline_intro_screen_btn, update_storyline_intro_screen_text,
    },
};
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Add the basic game state
        app.add_state::<GameMenuState>();

        // Add the setup system
        app.add_system(setup_start_screen.in_schedule(OnEnter(GameMenuState::StartScreen)));
        app.add_system(
            setup_storyline_intro_screen.in_schedule(OnEnter(GameMenuState::StorylineIntro)),
        );

        // Add the cleanup system
        app.add_system(cleanup_start_screen.in_schedule(OnExit(GameMenuState::StartScreen)));
        app.add_system(
            cleanup_storyline_intro_screen.in_schedule(OnExit(GameMenuState::StorylineIntro)),
        );

        // Add the various update systems
        app.add_system(update_button_hover_state.in_set(OnUpdate(GameMenuState::StartScreen)));
        app.add_system(update_start_button.in_set(OnUpdate(GameMenuState::StartScreen)));
        app.add_system(update_quit_button.in_set(OnUpdate(GameMenuState::StartScreen)));

        app.add_system(update_button_hover_state.in_set(OnUpdate(GameMenuState::StorylineIntro)));
        app.add_systems((
            update_storyline_intro_screen_text.in_set(OnUpdate(GameMenuState::StorylineIntro)),
            update_storyline_intro_screen_btn.in_set(OnUpdate(GameMenuState::StorylineIntro)),
        ));
        debug!("MenuPlugin loaded");
    }
}

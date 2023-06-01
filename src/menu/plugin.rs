use super::{
    state::GameMenuState,
    systems::{
        cleanup_start_screen, setup_start_screen, update_button_hover_state, update_start_button,
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

        // Add the cleanup system
        app.add_system(cleanup_start_screen.in_schedule(OnExit(GameMenuState::StartScreen)));

        // Add the various update systems
        app.add_system(update_button_hover_state.in_set(OnUpdate(GameMenuState::StartScreen)));
        app.add_system(update_start_button.in_set(OnUpdate(GameMenuState::StartScreen)));
        debug!("MenuPlugin loaded");
    }
}

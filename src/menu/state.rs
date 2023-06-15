//! Defines a simple enum to represent the current state of the menu

use bevy::prelude::*;

/// Represents the current state of the menu
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
pub enum GameMenuState {
    /// Indicates that the initial start screen is currently shown
    #[default]
    StartScreen,
    /// Indicates that the storyline's intro should be shown
    StorylineIntro,
    /// Indicates that the menu is not show at all as the player is in-game
    InGame,
}

//! Defines various global constants that are relevant to the whole game

use bevy::prelude::*;

/// A struct defining all of the keymaps use to connect input to in-game actions
#[allow(clippy::upper_case_acronyms)]
pub struct KEYMAP {}

impl KEYMAP {
    /// Moving forward
    pub const FORWARD: KeyCode = KeyCode::W;
    /// Moving backward
    pub const BACKWARD: KeyCode = KeyCode::S;
    /// Moving left
    pub const LEFT: KeyCode = KeyCode::A;
    /// Moving right
    pub const RIGHT: KeyCode = KeyCode::D;
}

/// Defines the different draw layers
#[allow(non_camel_case_types)]
pub struct DRAW_LAYER {}

impl DRAW_LAYER {
    /// The base layer. Everything is draw on top of this.
    pub const BASE: f32 = 0.0;
    /// The map layer. All parts of the map are drawn on this layer.
    pub const MAP: f32 = 1.0;
    /// The entities layer. All entities (in-game) are drawn on this layer.
    /// Entities like the player and enemies exist on this layer.
    pub const ENTITIES: f32 = 2.0;
    /// The effects layer. All effects are draw on this layer over any entities.
    pub const EFFECTS: f32 = 3.0;
}

/// The number of pixels that a single meter is equivalent to
pub const PIXELS_PER_METER: f32 = 32.0;

/// Defines the logging filter used by the game
#[cfg(debug_assertions)]
pub const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,the_caverns=debug";

/// Defines the logging filter used by the game
#[cfg(not(debug_assertions))]
pub const LOG_FILTER: &str = "warn,wgpu_core=warn,wgpu_hal=warn,the_caverns=warn";

/// The storyline intro text defined as an array
pub const STORY_INTRO: [&str; 7] = [
    "A darkness has fallen upon this once beautiful land.",
    "What was once full of life is now permeated with the stench of death and decay",
    "Lake crystal clear and sky pure blue turned red by the smoke that chokes the air.",
    "Where there were once animals and people living in harmony,",
    "There are now only creatures of darkness plotting their evil machinations.",
    "Where there was once a great kingdon of dwarves, their halls filled with splendor,",
    "There is now only the remains of their dark and dusty halls...",
];

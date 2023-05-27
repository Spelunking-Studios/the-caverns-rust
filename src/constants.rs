//! Defines various global constants that are relevant to the whole game

use bevy::prelude::*;

pub struct KEYMAP {}

impl KEYMAP {
    pub const FORWARD: KeyCode = KeyCode::W;
    pub const BACKWARD: KeyCode = KeyCode::S;
    pub const LEFT: KeyCode = KeyCode::A;
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

pub const PIXELS_PER_METER: f32 = 32.0;

// Logging
#[cfg(debug_assertions)]
pub const LOG_FILTER: &str = "info,wgpu_core=warn,wgpu_hal=warn,the_caverns=debug";

#[cfg(not(debug_assertions))]
pub const LOG_FILTER: &str = "warn,wgpu_core=warn,wgpu_hal=warn,the_caverns=warn";

use bevy::prelude::*;

pub struct KEYMAP {}

impl KEYMAP {
    pub const FORWARD: KeyCode = KeyCode::W;
    pub const BACKWARD: KeyCode = KeyCode::S;
    pub const LEFT: KeyCode = KeyCode::A;
    pub const RIGHT: KeyCode = KeyCode::D;
}

pub const PIXELS_PER_METER: f32 = 100.0;

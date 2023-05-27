//! # Tiles
//!
//! Defines component types for the various tiles supported by the game

use bevy::prelude::*;

trait MapTile {}

#[derive(Debug, Component)]
pub struct UnknownTile {
    pos: (i32, i32),
    offset: (u32, u32),
}

#[derive(Debug, Resource)]
pub struct UnknowTileTexture {
    pub loaded: bool,
    pub handle: Handle<Image>
}

impl MapTile for UnknownTile {}

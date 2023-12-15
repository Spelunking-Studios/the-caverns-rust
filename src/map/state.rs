use super::asset::MapAsset;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Resource, Default)]
pub struct MapState {
    pub handle: Handle<MapAsset>,
    pub textures: HashMap<String, Handle<Image>>,
    /// A [HashMap] storing the key of a texture atlas.
    pub texture_atlases: HashMap<i32, Handle<TextureAtlas>>,
    /// A HashMap storing the key and offset that maps to a specific texture
    ///
    /// The key "cave_dirt_floor" might map to ("Main Tileset", 32, 32) which
    /// would indicate that the texture for the "cave_dirt_floor" tile is
    /// located in the "Main Tileset" texture with and offset of (32, 32).
    pub texture_maps: HashMap<String, (String, u32, u32)>,
    pub current_level: u32,
    pub levels: Vec<LevelState>,
}

#[derive(Debug, Resource)]
pub struct LevelState {
}

/// Represents the various stages of readiness for a map
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum MapReadinessState {
    /// Map is currently loading
    Loading,

    /// Map is currently unloading
    Unloading,

    /// The map is loaded and ready for processing
    Loaded,

    /// The map is unloaded and can be replaced with another map
    #[default]
    Unloaded,

    /// The map is ready for gameplay
    Ready,
}

/// Represents the various states a level can be in
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum LevelReadinessState {
    // The level is unloaded and can be replaced with a different one
    #[default]
    Unloaded,

    // The level is currently loading and unfit for gameplay
    Loading,

    // The level is ready for gameplay
    Loaded,

    // The level is currently unloading and unfit for gameplay
    Unloading
}

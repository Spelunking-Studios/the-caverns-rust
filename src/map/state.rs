use super::asset::MapAsset;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Resource, Default)]
pub struct MapState {
    pub handle: Handle<MapAsset>,
    pub textures: HashMap<String, Handle<Image>>,
    /// A [HashMap] storing the key of a texture atlas.
    ///
    /// The key should correspond to the tileset's name.
    pub texture_atlases: HashMap<String, Handle<TextureAtlas>>,
    /// A HashMap storing the key and offset that maps to a specific texture
    ///
    /// The key "cave_dirt_floor" might map to ("Main Tileset", 32, 32) which
    /// would indicate that the texture for the "cave_dirt_floor" tile is
    /// located in the "Main Tileset" texture with and offset of (32, 32).
    pub texture_maps: HashMap<String, (String, u32, u32)>,
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

//! # Loader
//!
//! Module responsible for loading and unloading maps.
//!
//! ## State Flow
//!
//! There are 5 states a map can be in and are listed in the order they occur:
//! 1. [Loading](MapReadinessState::Loading):
//!    The map is loading from the asset file.
//! 2. [Loaded](MapReadinessState::Loaded):
//!    The map is loaded and now all the various entities are spawned. The
//!    map is not interactable yet though.
//! 3. [Ready](MapReadinessState::Ready):
//!    The map is ready to be played upon. All of the gameplay will happen in
//!    this state. The map is also now interactable.
//! 4. [Unloading](MapReadinessState::Unloading):
//!    The map is unloading. All entities are either destroyed or frozen via
//!    serialization. The map is once again isn't interactable.
//! 5. [Unloaded](MapReadinessState::Unloaded):
//!    Any previously loaded map is now fully unloaded. This is the "resting"
//!    state.
//!
//! ## State Switching
//!
//! ### Unloaded to Loading
//!
//! By default, the map is in the [Unloaded](MapReadinessState::Unloaded) state.
//! This the state the game begins. When a map is loaded through the asset system,
//! the caller is responsible for placing the map into the [Loading](MapReadinessState::Loading)
//! state.
//!
//! ```rust
//! // Example
//!
//! // Start loading the map
//! map_state.handle = asset_server.load("tiled/test.tmx");
//! // Switch to the Loading state
//! next_map_readiness.set(MapReadinessState::Loading);
//! ```
//!
//! ### Loading to Loaded
//!
//! Due to limitations in how asset loaders work, entering the
//! [Loading](MapReadinessState::Loading) state will trigger a system that will
//! run once the map asset has finished loading, and will place the map into
//! the [Loaded](MapReadinessState::Loaded) state.

use super::{
    asset::MapAsset,
    state::{MapReadinessState, MapState},
    util::map_cord_to_world_cord,
};
use crate::constants::{DRAW_LAYER, PIXELS_PER_METER};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::path::PathBuf;

/// Sets up the world's current map.
///
/// This system should only run when a new map has been loaded.
/// It is responsible for taking a Tiled Map and turning it into entities.
#[allow(clippy::map_entry)]
pub fn setup_map(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    mut next_map_readiness: ResMut<NextState<MapReadinessState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map_server: Res<Assets<MapAsset>>,
    texture_server: Res<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    // Switch into the Loaded state since the asset loader can't do that on its own
    next_map_readiness.set(MapReadinessState::Loaded);

    // Attempt to grab the map asset
    let map_asset = map_server.get(&map_state.handle);

    // We can't work on a non-existant map
    if map_asset.is_none() {
        warn!("MapReadinessState is Loading yet the map asset is None.");
        return;
    }

    // Actually get the map
    let map = map_asset.unwrap();

    info!(
        "Loading map {}",
        match &map.name {
            Some(name) => name.clone(),
            None => String::from("Not-Named"),
        }
    );

    info!("Loading tilesets");
    let tilesets = map.map.tilesets();

    for tileset in tilesets {
        debug!("Loading tileset {:?}", tileset.name);

        let tiled_image = tileset.image.as_ref();
        if let Some(tiled_image) = tiled_image {
            let key = tileset.name.clone();

            // Start loading the texture and grab it's handle
            let image: Handle<Image> = asset_server
                .load::<Image, PathBuf>(tiled_image.source.clone().canonicalize().unwrap());

            // Add the handle to the textures so that it can be used later
            if !map_state.textures.contains_key(&key) {
                map_state.textures.insert(key.clone(), image.clone());
            }

            // If there is no existing texture atlas, then create a new one
            // and add it to the hashmap
            if !map_state.texture_atlases.contains_key(&key) {
                let atlas: TextureAtlas = TextureAtlas::from_grid(
                    image.clone(), // Pass a handle to the tileset img
                    Vec2::new(tileset.tile_width as f32, tileset.tile_height as f32),
                    tileset.columns as usize,
                    (tileset.tilecount / tileset.columns) as usize,
                    Some(Vec2::new(0.0, 0.0)),
                    Some(Vec2::new(0.0, 0.0)),
                );
                map_state
                    .texture_atlases
                    .insert(key, texture_atlases.add(atlas));
            }
        }
    }

    info!("Tilesets loaded.");
    debug!(
        "Loaded {:?} texture atlases",
        map_state.texture_atlases.len()
    );

    // Process tile layers

    // Get a vector of all the layers in reverse order
    let mut layers: Vec<tiled::Layer> = vec![];

    for layer in map.map.layers() {
        layers.push(layer); // Add the layer to the end
        layers.rotate_right(1); // Rotate the vector so the last item is the first
    }

    // Get a iterable of all of the tile layers
    let tile_layers = layers
        .into_iter()
        .filter_map(|layer| match layer.layer_type() {
            tiled::LayerType::Tiles(layer) => Some(layer),
            _ => None,
        });

    // Process each tile layer
    for layer in tile_layers {
        let width = layer.width().unwrap();
        let height = layer.height().unwrap();

        for x in 0..width {
            for y in 0..height {
                let tile = layer.get_tile(x as i32, y as i32);

                // Continue processing the next tile if the current tile
                // is blank/doesn't exist
                if tile.is_none() {
                    continue;
                }

                // We know it's safe now to unwrap and we can shadow the first
                // tile variable for simplicity
                let tile = tile.unwrap();

                // The texture atlas that corresponds with this tile is just
                // the name of it's tileset
                let texture_key = tile.get_tileset().name.clone();

                // Grab the tile's id which maps as the texture index
                // within the tile's tileset
                let texture_index: usize = tile.id() as usize;

                // Get a handle to the texture atlas for this tileset
                let atlas_handle = map_state
                    .texture_atlases
                    .get(texture_key.as_str())
                    .unwrap_or_else(|| {
                        panic!(
                            "Texture atlases should have entry for key '{}'!",
                            &texture_key
                        )
                    });

                // Spawn the tile
                commands.spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: texture_index,
                        custom_size: Some(Vec2::new(PIXELS_PER_METER, PIXELS_PER_METER)),
                        anchor: bevy::sprite::Anchor::BottomLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(
                        x as f32 * PIXELS_PER_METER,
                        map_cord_to_world_cord(&map.map, y) as f32 * PIXELS_PER_METER,
                        DRAW_LAYER::MAP,
                    )),
                    texture_atlas: atlas_handle.clone(),
                    ..default()
                });
            }
        }
    }

    let width = map.map.width * map.map.tile_width;
    let height = map.map.height * map.map.tile_height;
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(width as f32, height as f32)),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, DRAW_LAYER::BASE)),
        ..default()
    });
}

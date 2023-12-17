use super::{
    asset::MapAsset,
    state::{LevelReadinessState, MapState},
    tiles::PlayerSpawnTile,
};
use crate::constants::{DRAW_LAYER, PIXELS_PER_METER};
use crate::player::Player;
use bevy::prelude::*;
use bevy::render::texture::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy_rapier2d::prelude::*;

/// Loads in the current level.
///
/// Handler for the [LevelReadinessState::Loading] state.
pub fn load_level(
    mut commands: Commands,
    mut map_state: ResMut<MapState>,
    mut next_level_readiness: ResMut<NextState<LevelReadinessState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map_server: Res<Assets<MapAsset>>,
    mut images: ResMut<Assets<Image>>,
    mut player_position: Query<&mut Transform, (With<RigidBody>, With<Player>)>,
) {
    debug!("Loading level");

    // Attempt to grab the map asset
    let map_asset = map_server.get(&map_state.handle);

    // We can't work on a non-existant map
    if map_asset.is_none() {
        warn!("Map asset is missing.");
        return;
    }

    // Actually get the map
    let map = map_asset.unwrap();

    let current_ldtk_level = &map.project.levels[map_state.current_level as usize];

    // The project structure defines different layers that each level can have
    // We can go through each of these layers and initialize their corresponding in-game layer
    if let Some(layer_instances) = current_ldtk_level.layer_instances.as_ref() {
        for layer in layer_instances {
            match layer.__type.as_str() {
                "Tiles" => {
                    // Grab the handle to the texture to we can load it
                    let texture_atlas_handle = map_state
                        .texture_atlases
                        .get(&layer.__tileset_def_uid.unwrap());

                    if texture_atlas_handle.is_none() {
                        error!(
                            "Texture atlas for tileset {} is missing!",
                            layer.__tileset_def_uid.unwrap()
                        );
                        continue;
                    }

                    // Get the actual texture from bevy
                    let texture_atlas = texture_atlases.get(texture_atlas_handle.unwrap().id());

                    if texture_atlas.is_none() {
                        error!(
                            "Texture atlas for tileset {} is missing!",
                            layer.__tileset_def_uid.unwrap()
                        );
                        continue;
                    }

                    // Some nameshadowing to make things more readable below
                    let texture_atlas = texture_atlas.unwrap();
                    let texture_cols = texture_atlas.size[0] / 32.;

                    // Fix an annoying rendering bug
                    fix_rendering_bug(&mut images, texture_atlas.texture.clone());

                    // Spawn in each tile
                    for tile in &layer.grid_tiles {
                        let layer_offset = &tile.px;
                        let texture_offset = &tile.src;
                        let texture_index =
                            (tile.src[0] / 32) as f32 + (tile.src[1] / 32) as f32 * texture_cols;

                        let tile_id = commands
                            .spawn(SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    index: texture_index as usize,
                                    custom_size: Some(Vec2::new(
                                        PIXELS_PER_METER,
                                        PIXELS_PER_METER,
                                    )),
                                    anchor: bevy::sprite::Anchor::Center,
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    (layer_offset[0] / 32) as f32 * PIXELS_PER_METER,
                                    -1. * (layer_offset[1] / 32) as f32 * PIXELS_PER_METER,
                                    DRAW_LAYER::MAP,
                                )),
                                texture_atlas: texture_atlas_handle.unwrap().clone(),
                                ..default()
                            })
                            .id();

                        if tile.t == 2 {
                            commands.entity(tile_id).insert(PlayerSpawnTile::default());
                            for mut position in &mut player_position {
                                position.translation.x =
                                    (layer_offset[0] / 32) as f32 * PIXELS_PER_METER;
                                position.translation.y =
                                    -1. * (layer_offset[1] / 32) as f32 * PIXELS_PER_METER;
                            }
                        }
                    }
                }
                "IntGrid" => {
                    debug!("IntGrid");
                }
                _ => {}
            }
        }
    } else {
        warn!("No layers for level {}", map_state.current_level);
        return;
    }

    next_level_readiness.set(LevelReadinessState::Loaded);
}

/// Fixes a incredibly annoying rendering bug in Bevy by adjusting the image sampler for the tile
/// textures.
///
/// See [this](https://github.com/bevyengine/bevy/discussions/1289#discussioncomment-304058)
/// comment on a GitHub discussion
fn fix_rendering_bug(images: &mut ResMut<Assets<Image>>, texture: Handle<Image>) {
    let image_asset = images.get_mut(texture.id());

    if let Some(image) = image_asset {
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            min_filter: ImageFilterMode::Nearest,
            ..default()
        });
    } else {
        warn!("Failed to fix rendering bug for an image!");
    }
}

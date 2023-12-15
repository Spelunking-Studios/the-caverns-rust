use super::{
    asset::{MapAsset, MapAssetLoader},
    level::load_level,
    loader::setup_map,
    state::{MapReadinessState, LevelReadinessState, MapState},
};
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct MapPlugin {}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MapReadinessState>();
        app.add_state::<LevelReadinessState>();
        app.add_systems(OnEnter(MapReadinessState::Loading), setup_map);
        app.add_systems(OnEnter(LevelReadinessState::Loading), load_level);

        app.insert_resource(MapState { ..default() });

        app.register_asset_loader(MapAssetLoader)
            .init_asset::<MapAsset>();

        // Strange rendering issue
        app.insert_resource(Msaa::Off);

        debug!("MapPlugin loaded");
    }
}

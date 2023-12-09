use super::{
    asset::{MapAsset, MapAssetLoader},
    loader::setup_map,
    state::{MapReadinessState, MapState},
};
use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct MapPlugin {}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MapReadinessState>();
        app.add_systems(OnEnter(MapReadinessState::Loading), setup_map);

        app.insert_resource(MapState { ..default() });

        app.add_asset::<MapAsset>();
        app.init_asset_loader::<MapAssetLoader>();
        debug!("MapPlugin loaded");
    }
}

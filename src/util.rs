use crate::{map::state::MapState, player::spawn_player};
use bevy::prelude::*;

/// Utility function to handle calling the various functions needed to get the
/// game going
pub fn start_game(
    commands: &mut Commands,
    map_state: &mut ResMut<MapState>,
    asset_server: &Res<AssetServer>,
) {
    map_state.handle = asset_server.load("tiled/test.tmx");
    spawn_player(commands);
}

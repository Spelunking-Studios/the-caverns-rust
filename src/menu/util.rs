use bevy::prelude::*;
use crate::{
    map::state::MapState,
    player::spawn_player
};

pub fn start_game(
    commands: &mut Commands,
    map_state: &mut ResMut<MapState>,
    asset_server: &Res<AssetServer>
) {
    map_state.handle = asset_server.load("tiled/test.tmx");
    spawn_player(commands);
}

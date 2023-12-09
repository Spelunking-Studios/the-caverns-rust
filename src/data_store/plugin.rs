//! Plugin for the data store module

use bevy::prelude::*;

/// Struct for the data store plugin
#[derive(Debug, Default)]
pub struct DataStorePlugin;

impl Plugin for DataStorePlugin {
    fn build(&self, app: &mut App) {}
}

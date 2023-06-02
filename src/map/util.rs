//! Utilities for the map module

/// Converts the provided map coordinate to a world coordinate
/// based upon the provided map.
///
/// > **Warning**:
/// > This function only operates on the Y axis and as such, shouldn't be used with
/// any other values.
///
/// The coordinate is a map style coordinate used by tiled,
/// and the function will return a world style coordinate used by bevy.
pub fn map_cord_to_world_cord(map: &tiled::Map, cord: u32) -> u32 {
    map.height - 1 - cord
}

//! Generally used components such as [Speed]

use bevy::prelude::*;

/// A simple component that represents the speed of something
///
/// You can get the speed by just getting the property `0`.
///
/// > Note: The speed should be in meters per second (i.e. this isn't a speed in
/// pixels per second).
///
/// ## Example
///
/// ```rust
/// fn some_fn(mut commands: Commands) {
///     commands.spawn((
///         SomeEntity {
///             // Some code goes here
///             ..default()
///         },
///         Speed(1.0)
///     ));
/// }
/// ```
#[derive(Component)]
pub struct Speed(pub f32);

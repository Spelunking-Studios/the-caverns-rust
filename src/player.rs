use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::constants::{
    DRAW_LAYER,
    KEYMAP
};
use crate::general::components::Speed;

pub const PLAYER_SPEED: f32 = 200.0;

// Player component (marker)
#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    p: Player,
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(64., 64., DRAW_LAYER::ENTITIES)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        })
        .insert(GravityScale(0.0))
        .insert(Sleeping::disabled())
        .insert(Collider::cuboid(16.0, 16.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 1.0,
        })
        .insert(Speed(PLAYER_SPEED))
        .insert(Player);
}

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut players: Query<(&Speed, &mut Velocity, &Transform), With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let mut camera_transform = cameras.single_mut();
    for (speed, mut velocity, transform) in players.iter_mut() {
        // Start Moving
        if keys.pressed(KEYMAP::FORWARD) {
            velocity.linvel.y = speed.0;
        }
        if keys.pressed(KEYMAP::BACKWARD) {
            velocity.linvel.y = -1.0 * speed.0;
        }
        if keys.pressed(KEYMAP::LEFT) {
            velocity.linvel.x = -1.0 * speed.0;
        }
        if keys.pressed(KEYMAP::RIGHT) {
            velocity.linvel.x = speed.0;
        }

        // Stop moving
        if keys.just_released(KEYMAP::FORWARD) || keys.just_released(KEYMAP::BACKWARD) {
            velocity.linvel.y = 0.0;
        }

        if keys.just_released(KEYMAP::LEFT) || keys.just_released(KEYMAP::RIGHT) {
            velocity.linvel.x = 0.0;
        }

        // Move the camera to match the player's position
        camera_transform.translation.y = transform.translation.y;
        camera_transform.translation.x = transform.translation.x;
    }
}

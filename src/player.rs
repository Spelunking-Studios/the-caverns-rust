use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::general::components::Speed;

pub const PLAYER_BASE_SPEED: f32 = 500.0;

// Player component (marker)
#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0
        })
        .insert(GravityScale(0.0))
        .insert(Sleeping::disabled())
        .insert(Collider::cuboid(25.0, 25.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Damping { linear_damping: 0.5, angular_damping: 1.0 })
        .insert(Speed(PLAYER_BASE_SPEED))
        .insert(Player);
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut players: Query<(&Speed, &mut Velocity, &Transform), With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let mut camera_transform = cameras.single_mut();
    for (speed, mut velocity, transform) in players.iter_mut() {
        if keys.pressed(KeyCode::W) {
            velocity.linvel.y += speed.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::S) {
            velocity.linvel.y -= speed.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::A) {
            velocity.linvel.x -= speed.0 * time.delta_seconds();
        }
        if keys.pressed(KeyCode::D) {
            velocity.linvel.x += speed.0 * time.delta_seconds();
        }

        // Move the camera to match the player's position
        camera_transform.translation.y = transform.translation.y;
        camera_transform.translation.x = transform.translation.x;
    }
}

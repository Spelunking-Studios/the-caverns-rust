use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Obstacle;

pub fn spawn_obstacle(
    commands: &mut Commands,
    color: Color,
    size: Option<Vec2>,
    transform: Transform,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: size,
                ..default()
            },
            transform,
            ..default()
        })
        .insert(Obstacle)
        .insert(RigidBody::Fixed)
        .insert(GravityScale(0.0))
        .insert(Sleeping::disabled())
        .insert(Collider::cuboid(
            size.unwrap().x / 2.0,
            size.unwrap().y / 2.0,
        ))
        .insert(transform);
}

use bevy::{
    prelude::{Color, Commands, Transform, Vec2},
    sprite::{Sprite, SpriteBundle},
};
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, Velocity};

pub fn spawn_map(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 5.)),
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -100., 0.),
            ..Default::default()
        },
        RigidBody::Fixed,
        Velocity::default(),
        Collider::cuboid(100., 2.5),
        LockedAxes::ROTATION_LOCKED_Z,
    ));
}

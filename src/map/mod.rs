use bevy::{
    prelude::{Color, Commands, Transform, Vec2, AssetServer, Res},
    sprite::{Sprite, SpriteBundle},
};
use bevy_ecs_ldtk::LdtkWorldBundle;
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, Velocity, GravityScale};

use crate::entities::{blocks::{BlockBundle, WallBundle}, collision::CollisionBundle};

pub fn spawn_map(mut commands: Commands) {
    let floor = WallBundle::new( BlockBundle::new(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(200., 5.)),
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -100., 0.),
            ..Default::default()
        },
        CollisionBundle::new(RigidBody::Fixed, Collider::cuboid(100., 2.5), LockedAxes::ROTATION_LOCKED_Z, Velocity::default(), GravityScale(0.))));

    let block = BlockBundle::new(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20., 20.)),
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -20., 0.),
            ..Default::default()
        },
        CollisionBundle::new(RigidBody::Fixed, Collider::cuboid(10., 10.), LockedAxes::ROTATION_LOCKED_Z, Velocity::default(), GravityScale(0.0)));


    commands.spawn(floor);
    commands.spawn(block);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
   let ldtk_handle = asset_server.load("/usr/share/ldtk/extraFiles/samples/Typical_2D_platformer_example.ldtk"); 
    commands.spawn(LdtkWorldBundle{
        ldtk_handle,
        ..Default::default()
    });
}

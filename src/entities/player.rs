use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, Input, KeyCode, Query, Res, ResMut, Transform,
        Vec2, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::Time,
};

use super::animations::{FrameTime, SpriteAnimation};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
        Vec2::splat(32.),
        11,
        1,
        None,
        None,
    );
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: texture_atlas.add(atlas),
            ..SpriteSheetBundle::default()
        },
        Player,
        SpriteAnimation {
            len: 11,
            frame_time: 1. / 20.,
        },
        FrameTime(0.0),
    ));
}

pub const MOVE_SPEED: f32 = 100.0;

pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = player.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        player.translation.y += MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        player.translation.y -= MOVE_SPEED * time.delta_seconds();
    }
}

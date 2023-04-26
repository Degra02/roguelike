use bevy::{
    prelude::{
        AssetServer, Assets, Component, Handle, Input, KeyCode, Query, Res, ResMut, Vec2, With,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
    time::Time,
};

use super::player::Player;

#[derive(Component)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
}

#[derive(Component)]
pub struct FrameTime(pub f32);

pub fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            sprite.index %= animation.len;
            frame_time.0 -= animation.frame_time;
        }
    }
}

pub fn change_player_animation(
    mut player: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut SpriteAnimation,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();

    if input.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Run (32x32).png"),
            Vec2::splat(32.),
            12,
            1,
            None,
            None,
        );

        *atlas = texture_atlas.add(new_atlas);
        animation.len = 12;
        sprite.index = 0;
    }

    if input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        && !input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
    {
        let new_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Charactes/Mask Dude/Idle (32x32).png"),
            Vec2::splat(32.),
            11,
            1,
            None,
            None,
        );

        *atlas = texture_atlas.add(new_atlas);
        animation.len = 11;
        sprite.index = 0;
    }

    if input.any_just_pressed([KeyCode::A]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::D]) && !input.any_just_pressed([KeyCode::A]) {
        sprite.flip_x = false;
    } else if input.any_just_released([KeyCode::A])
        && !input.any_pressed([KeyCode::A])
        && input.any_pressed([KeyCode::D])
    {
        sprite.flip_x = false;
    }
}

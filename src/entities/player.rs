use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, Input, KeyCode, Query, Res, ResMut, Transform,
        Vec2, With, error,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::Time,
};

use super::animations::{sprite_animation::{FrameTime, SpriteAnimation}, player_animations::{PlayerAnimations, Animation}, self};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    animations: Res<PlayerAnimations>
) {
    let Some((texture_atlas, animation)) = animations.get(Animation::Idle) else { error!("Failed to find animation: Idle"); return;};
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas,
            ..SpriteSheetBundle::default()
        },
        Player,
        animation,
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

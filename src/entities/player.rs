use bevy::{
    prelude::{
        AssetServer, Assets, Commands, Component, Input, KeyCode, Query, Res, ResMut, Transform,
        Vec2, With, error, Entity, Without,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::Time,
};
use leafwing_input_manager::{Actionlike, prelude::InputMap};

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

#[derive(Debug, Actionlike, Clone)]
pub enum PlayerInput {
    Left, Right, Jump
}

impl PlayerInput {
    pub fn player_one() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([(KeyCode::A, PlayerInput::Left), (KeyCode::D, PlayerInput::Right), (KeyCode::Space, PlayerInput::Jump)]);

        map
    }
}

pub const MOVE_SPEED: f32 = 300.0;

pub fn move_player(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform), With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>
) {
    let (entity, mut player) = player.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVE_SPEED * time.delta_seconds();
    } else if input.any_just_pressed([KeyCode::Space]) {
       commands.entity(entity).insert(Jump(100.)); 
    }
}

#[derive(Component)]
pub struct Jump(f32);

const GRAVITY: f32 = 700.0;

pub fn player_jump(
    mut commands: Commands,
    time: Res<Time>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>
) {
   let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {return;}; 
    let jump_power = (time.delta_seconds() * GRAVITY * 2.).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<Jump>();
    } 
}

pub fn player_fall(
    time: Res<Time>,
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>
) {
    let Ok(mut player) = player.get_single_mut() else {return;};
    if player.translation.y > 0. {
        player.translation.y -= time.delta_seconds() * GRAVITY;
        if player.translation.y < 0. {player.translation.y = 0.0;}
    }       
}

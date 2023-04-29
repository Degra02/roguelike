use bevy::{
    prelude::{
        Commands, Component, Input, KeyCode, Query, Res, Transform, With, error, Entity, Without, Vec2, Vec3,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::Time,
};
use leafwing_input_manager::{Actionlike, prelude::{InputMap, ActionState}};

use crate::animations::{player_animations::{PlayerAnimations, Animation}, sprite_animation::FrameTime};

use super::hit_box::{HitBox, Grounded, check_hit};

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
        Grounded(true),
        HitBox(Vec2::splat(32.))
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
    mut player: Query<(Entity, &mut Transform, &Grounded, &HitBox, &ActionState<PlayerInput>), With<Player>>,
    hitboxes: Query<(&HitBox, &Transform), Without<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>
) {
    let (entity, mut p_offset, grounded, &p_hitbox,_p_input) = player.single_mut();
    let delat = if input.any_just_pressed([KeyCode::Space]) && grounded.0 {
        commands.entity(entity).insert(Jump(100.));
        return;
    } else if input.any_pressed([KeyCode::A]) {
        -MOVE_SPEED * time.delta_seconds()* (0.5 + (grounded.0 as u16) as f32) 
    } else if input.any_pressed([KeyCode::D]) {
        MOVE_SPEED * time.delta_seconds()* (0.5 + (grounded.0 as u16) as f32)  
    } else {
        return;
    };

    let new_pos = p_offset.translation + Vec3::X * delat;
    for (&hitbox, offset) in &hitboxes {
        if check_hit(p_hitbox, p_offset.translation, hitbox, offset.translation) {
            return;
        }
    }

    p_offset.translation = new_pos;
}

#[derive(Component)]
pub struct Jump(f32);

const GRAVITY: f32 = 300.0;

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
    mut player: Query<(&mut Transform, &HitBox), With<Player>>,
    hitboxes: Query<(&HitBox, &Transform), Without<Player>>
) {   
    let Ok((mut p_offset, &p_hitbox)) = player.get_single_mut() else {return;};
    let new_pos = p_offset.translation - Vec3::Y * GRAVITY * time.delta_seconds();
    for (&hitbox, offset) in &hitboxes {
        if check_hit(p_hitbox, new_pos, hitbox, offset.translation) {return;}
    }
    p_offset.translation = new_pos;
}

use super::{
    health::Health, hit_box::Grounded,
};
use crate::animations::{
    player_animations::{Animation, PlayerAnimations},
    sprite_animation::FrameTime,
};
use bevy::{
    prelude::{
        error, Bundle, Changed, Commands, Component, Entity, KeyCode, Query, Res, With,
    },
    reflect::Reflect,
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, Velocity};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    _p: Player,

    #[bundle]
    sprite: SpriteSheetBundle,
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, animations: Res<PlayerAnimations>) {
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
        InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..Default::default()
        },
        Jump(false),
        RigidBody::Dynamic,
        Velocity::default(),
        Collider::cuboid(9., 16.),
        LockedAxes::ROTATION_LOCKED_Z,
    ));
}

#[derive(Debug, Actionlike, Clone)]
pub enum PlayerInput {
    Left,
    Right,
    Jump,
    Fall,
}

impl PlayerInput {
    pub fn player_one() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            (KeyCode::A, PlayerInput::Left),
            (KeyCode::D, PlayerInput::Right),
            (KeyCode::Space, PlayerInput::Jump),
        ]);

        map
    }
}

pub const MOVE_SPEED: f32 = 300.0;

pub fn move_player(
    mut player: Query<(&mut Velocity, &ActionState<PlayerInput>, &Grounded), With<Player>>,
) {
    let (mut velocity, input, grounded) = player.single_mut();

    if input.just_pressed(PlayerInput::Jump) && grounded.0 {
        velocity.linvel.y = 600.;
    } else if input.just_pressed(PlayerInput::Fall) {
        velocity.linvel.y = velocity.linvel.y.min(0.);
    } else if input.just_pressed(PlayerInput::Left) {
        velocity.linvel.x = -MOVE_SPEED;
    } else if input.just_pressed(PlayerInput::Right) {
        velocity.linvel.x = MOVE_SPEED;
    } else if input.just_released(PlayerInput::Left) || else if input.just_released(PlayerInput::Right) {
        velocity.linvel.x = 0.;
    }
}

#[derive(Component, Reflect)]
pub struct Jump(bool);

pub fn double_jump(
    mut player: Query<(&mut Jump, &mut Velocity, &ActionState<PlayerInput>), With<Player>>,
    can_jump: Query<(Entity, &Grounded), Changed<Grounded>>,
) {
    for (entity, grounded) in &can_jump {
        if let Ok((mut jump, _, _)) = player.get_mut(entity) {
            if grounded.0 {
                jump.0 = true;
            }
        }
    }

    for (mut jump, mut velocity, input) in player.iter_mut() {
        if velocity.linvel.y.abs() < 0.01 {
            return;
        }
        if input.just_pressed(PlayerInput::Jump) && jump.0 {
            jump.0 = false;
            velocity.linvel.y = 100.;
        }
    }
}

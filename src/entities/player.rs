use std::time::Duration;

use super::{
    health::Health, collision::{Grounded, CollisionBundle},
};
use crate::animations::{
    player_animations::{Animation, PlayerAnimations},
    sprite_animation::{FrameTime, SpriteAnimation},
};
use bevy::{
    prelude::{
        error, Bundle, Changed, Commands, Component, Entity, KeyCode, Query, Res, With, Vec2,
    },
    reflect::Reflect,
    sprite::{SpriteSheetBundle, TextureAtlasSprite}, time::Time, ecs::schedule::MainThreadExecutor,
};
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, Velocity, KinematicCharacterController, GravityScale, AdditionalMassProperties, KinematicCharacterControllerOutput};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    _p: Player,
    controller: KinematicCharacterController,
    animation: SpriteAnimation,
    frame_time: FrameTime,

    #[bundle]
    input_manager: InputManagerBundle<PlayerInput>,

    #[bundle]
    sprite: SpriteSheetBundle,

    #[bundle]
    collision: CollisionBundle
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, animations: Res<PlayerAnimations>) {
    let Some((texture_atlas, animation)) = animations.get(Animation::Idle) else { error!("Failed to find animation: Idle"); return;};

    let player_bundle = PlayerBundle {
        health: Health::new(4),
        _p: Player,
        animation,
        frame_time: FrameTime(0.0),
        controller: KinematicCharacterController::default(),
        input_manager: InputManagerBundle {
            input_map: PlayerInput::player_one(),
            ..Default::default()
        },
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas,
            ..SpriteSheetBundle::default()
        },
        collision: CollisionBundle::new(RigidBody::Dynamic, Collider::cuboid(9., 16.), LockedAxes::ROTATION_LOCKED_Z, Velocity::default()),
    };

    commands.spawn(player_bundle);
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
            (KeyCode::S, PlayerInput::Fall)
        ]);

        map
    }
}

pub const MOVE_SPEED: f32 = 300.0;
pub const JUMP_FORCE: f32 = 500.0;

pub fn move_player(
    mut player: Query<(&mut Velocity, &ActionState<PlayerInput>), With<Player>>,
) {
    let (mut velocity, input) = player.single_mut();

    if input.just_pressed(PlayerInput::Left) {
        velocity.linvel.x = -MOVE_SPEED;
    } else if input.just_pressed(PlayerInput::Right) {
        velocity.linvel.x = MOVE_SPEED;
    } else if input.just_released(PlayerInput::Left) || input.just_released(PlayerInput::Right) {
        velocity.linvel.x = 0.;
    }
}

#[derive(Component, Reflect)]
pub struct Jump(bool);

pub fn jump(
    mut input_query: Query<&ActionState<PlayerInput>, With<Player>>,
    mut controllers: Query<(&mut KinematicCharacterController, &KinematicCharacterControllerOutput, &Velocity), With<Player>>,
    mut commands: Commands,
    time: Res<Time>
) {
    for input in input_query.iter() {
        for (mut controller, k_output, velocity) in controllers.iter_mut() {
            match k_output.grounded {
                true => if input.just_pressed(PlayerInput::Jump) {
                    controller.translation = match controller.translation {
                        Some(mut v) => {v.y += 20.0; Some(v)},
                        None => Some(Vec2::new(0.0, 20.0))
                    }
                } else {
                        controller.translation = match controller.translation {
                            Some(mut v) => {v.y = -4.0; Some(v)},
                            None => Some(Vec2::new(0.0, -4.0)),
                        }
                    }, 
                false => {
                    if input.just_released(PlayerInput::Jump) {
                        controller.translation = match controller.translation {
                            Some(mut v) => {v.y = -8.0; Some(v)},
                            None => Some(Vec2::new(0.0, -8.0)),
                        }
                    } else if input.pressed(PlayerInput::Jump) {
                        let has_held_jump_for_duration = input.current_duration(PlayerInput::Jump);
                        if has_held_jump_for_duration >= Duration::from_secs(2) {
                            controller.translation = match controller.translation {
                                Some(mut v) => {v.y = -8.0; Some(v)},
                                None => Some(Vec2::new(0.0, -8.0)),
                            }
                        }
                    }
                },
            } 
        }  
    }


}

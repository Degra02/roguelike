

use super::{
    health::Health, collision::CollisionBundle,
};
use crate::animations::{
    player_animations::{Animation, PlayerAnimations},
    sprite_animation::{FrameTime, SpriteAnimation},
};
use bevy::{
    prelude::{
        error, Bundle, Commands, Component, KeyCode, Query, Res, With, Vec2, Transform,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite}, time::Time,
};
use bevy_rapier2d::prelude::{Collider, LockedAxes, RigidBody, Velocity, KinematicCharacterController, KinematicCharacterControllerOutput, GravityScale};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    _p: Player,
    animation: SpriteAnimation,
    frame_time: FrameTime,

    controller: KinematicCharacterController,
    output: KinematicCharacterControllerOutput,

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
        output: KinematicCharacterControllerOutput::default(),
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
        collision: CollisionBundle::new(RigidBody::Dynamic, Collider::cuboid(9., 16.), LockedAxes::ROTATION_LOCKED_Z, Velocity::default(), GravityScale(1.0)),
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
pub const JUMP_FORCE: f32 = 80.0;

pub fn move_player(
    mut player: Query<(&mut Velocity, &ActionState<PlayerInput>), With<Player>>,
) {
    let (mut velocity, input) = player.single_mut();

    if input.just_pressed(PlayerInput::Left) || input.pressed(PlayerInput::Left){
        velocity.linvel.x = -MOVE_SPEED;
    } else if input.just_pressed(PlayerInput::Right) || input.pressed(PlayerInput::Right) {
        velocity.linvel.x = MOVE_SPEED;
    } else if input.just_released(PlayerInput::Left) || input.just_released(PlayerInput::Right) {
        velocity.linvel.x = 0.;
    }
}

pub fn jump(
    input_query: Query<&ActionState<PlayerInput>, With<Player>>,
    mut controllers: Query<(&mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut Velocity), With<Player>>,
    _commands: Commands,
    time: Res<Time>
) {
    for input in input_query.iter() {
        for (mut controller, k_output, mut velocity) in controllers.iter_mut() {
            match k_output.grounded {
                true => if input.pressed(PlayerInput::Jump) {
                    velocity.linvel.y += 50.0 * time.delta_seconds();
                } else {
                        controller.translation = match controller.translation {
                            Some(mut v) => {v.y = -14.0; Some(v)},
                            None => Some(Vec2::new(0.0, -14.0)),
                        }
                    }, 
                false => {
                    if input.just_pressed(PlayerInput::Jump) {
                        velocity.linvel.y += 25. * time.delta_seconds() * 1000.;
                    } else if input.pressed(PlayerInput::Jump){
                        velocity.linvel.y += 2. * time.delta_seconds() * 1000.;
                    } else if input.just_released(PlayerInput::Jump) {
                        velocity.linvel.y -= 5. * time.delta_seconds() * 1000.;
                    } 
                },
            } 
        }  
    }


}

pub fn check_borders(
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut controller = player.single_mut(); 

    if controller.translation.y < -400.0 {
        controller.translation.y = 400.0;
    }

}

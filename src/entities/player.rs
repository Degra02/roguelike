use super::{collision::CollisionBundle, health::Health};
use crate::{
    animations::{
        player_animations::{Animation, PlayerAnimations},
        sprite_animation::{FrameTime, SpriteAnimation},
    },
    CameraTest, AnimationPlugin,
};
use bevy::{
    prelude::{
        error, Bundle, Commands, Component, KeyCode, Query, Res, Transform, Vec2, Vec3,
        With, Without, App, Plugin,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::Time,
    window::Window, reflect::Reflect,
};
use bevy_rapier2d::prelude::{
    CharacterAutostep, CharacterLength, Collider, GravityScale, KinematicCharacterController,
    KinematicCharacterControllerOutput, LockedAxes, RigidBody, Velocity,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike, InputManagerBundle,
};

#[derive(Reflect, Component, Default, Debug, Clone)]
pub struct Jump(pub bool, f32);

#[derive(Reflect, Component, Default, Debug, Clone)]
pub struct Speed(pub f32);

#[derive(Bundle)]
pub struct PlayerBundle {
    health: Health,
    _p: Player,
    
    speed: Speed,

    animation: SpriteAnimation,
    frame_time: FrameTime,

    jump: Jump,

    controller: KinematicCharacterController,
    output: KinematicCharacterControllerOutput,

    #[bundle]
    input_manager: InputManagerBundle<PlayerInput>,

    #[bundle]
    sprite: SpriteSheetBundle,

    #[bundle]
    collision: CollisionBundle,
}

#[derive(Component, Reflect, Default, Debug, Clone)]
pub struct Player;

pub fn spawn_player(mut commands: Commands, animations: Res<PlayerAnimations>) {
    let Some((texture_atlas, animation)) = animations.get(Animation::Idle) else { error!("Failed to find animation: Idle"); return;};

    let player_bundle = PlayerBundle {
        health: Health::new(4),
        _p: Player,
        speed: Speed(800.),
        animation,
        frame_time: FrameTime(0.0),
        jump: Jump(false, 100.),
        controller: KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(0.2),
                min_width: CharacterLength::Relative(0.0),
                include_dynamic_bodies: true,
            }),
            ..Default::default()
        },
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
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                // scale: Vec3::new(0.25, 0.25, 1.),
                ..Default::default()
            },
            ..SpriteSheetBundle::default()
        },
        collision: CollisionBundle::new(
            RigidBody::Dynamic,
            Collider::cuboid(36., 50.),
            LockedAxes::ROTATION_LOCKED_Z,
            Velocity::default(),
            GravityScale(1.0),
        ),
    };

    commands.spawn(player_bundle);
}

#[derive(Debug, Actionlike, Clone)]
pub enum PlayerInput {
    Left,
    Right,
    Jump,
    Fall,
    Crouch,
    CrouchWalkRight,
    CrouchWalkLeft,
    LookUp,
}

impl PlayerInput {
    pub fn player_one() -> InputMap<PlayerInput> {
        let mut map = InputMap::default();
        map.insert_multiple([
            (KeyCode::A, PlayerInput::Left),
            (KeyCode::D, PlayerInput::Right),
            (KeyCode::Space, PlayerInput::Jump),
            (KeyCode::S, PlayerInput::Crouch),
            (KeyCode::W, PlayerInput::LookUp),
        ]);

        map.insert_chord([KeyCode::S, KeyCode::D], PlayerInput::CrouchWalkRight);
        map.insert_chord([KeyCode::S, KeyCode::A], PlayerInput::CrouchWalkLeft);

        map
    }
}

pub const JUMP_FORCE: f32 = 80.0;

pub fn move_player(
    mut player: Query<(&mut Velocity, &Speed, &ActionState<PlayerInput>), With<Player>>,
    time: Res<Time>,
) {
    let (mut velocity, speed, input) = player.single_mut();

    if input.pressed(PlayerInput::CrouchWalkLeft) {
        velocity.linvel.x = -speed.0 * 0.3;
    } else if input.pressed(PlayerInput::CrouchWalkRight) {
        velocity.linvel.x = speed.0 * 0.3;
    } else if input.just_pressed(PlayerInput::Left) || input.pressed(PlayerInput::Left) {
        velocity.linvel.x = -speed.0;
    } else if input.just_pressed(PlayerInput::Right) || input.pressed(PlayerInput::Right) {
        velocity.linvel.x = speed.0;
    } else if input.just_released(PlayerInput::Left) {
        velocity.linvel.x += 30. * input.current_duration(PlayerInput::Left).as_secs_f32();
    } else if input.just_released(PlayerInput::Right) {
        velocity.linvel.x -= 3000. * time.delta_seconds();
    } else {
        velocity.linvel.x = 0.0;
    }
}

pub fn jump(
    input_query: Query<&ActionState<PlayerInput>, With<Player>>,
    mut controllers: Query<
        (
            &mut KinematicCharacterController,
            &KinematicCharacterControllerOutput,
            &mut Velocity,
            &mut Jump,
        ),
        With<Player>,
    >,
    _commands: Commands,
    time: Res<Time>,
) {
    for input in input_query.iter() {
        for (mut controller, k_output, mut velocity, mut jump) in controllers.iter_mut() {
            match k_output.grounded {
                true => {
                    if input.pressed(PlayerInput::Jump) {
                        velocity.linvel.y += jump.1 * time.delta_seconds();
                    } else {
                        controller.translation = match controller.translation {
                            Some(mut v) => {
                                v.y = -14.0;
                                Some(v)
                            }
                            None => Some(Vec2::new(0.0, -14.0)),
                        }
                    }
                }
                false => {
                    if input.just_pressed(PlayerInput::Jump) && !jump.0 {
                        velocity.linvel.y += jump.1 * time.delta_seconds() * 1000.;
                        jump.0 = true;
                    } else if input.pressed(PlayerInput::Jump)
                        && input.current_duration(PlayerInput::Jump).as_millis() < 180
                    {
                        velocity.linvel.y += 2. * time.delta_seconds() * 1000.;
                    } else if input.just_released(PlayerInput::Jump) {
                        velocity.linvel.y -= 5. * time.delta_seconds() * 1000.;
                    }

                    if velocity.linvel.y == 0.0 {
                        jump.0 = false;
                    }
                }
            }
        }
    }
}

pub fn look_up_down_handle(
    player: Query<&ActionState<PlayerInput>, With<Player>>,
    mut camera: Query<&mut Transform, With<CameraTest>>,
    time: Res<Time>,
) {
    let input = player.single();
    let mut camera = camera.single_mut();
    if input.pressed(PlayerInput::LookUp)
        && input.current_duration(PlayerInput::LookUp).as_secs_f32() > 0.7 {
        camera.translation.y += 3800.0 * time.delta_seconds();
    } else if input.just_released(PlayerInput::LookUp) && input.previous_duration(PlayerInput::LookUp).as_secs_f32() > 1.{
        camera.translation.y -= 3800.0 * time.delta_seconds();
    } else if input.pressed(PlayerInput::Crouch)
        && input.current_duration(PlayerInput::Crouch).as_secs_f32() > 0.7
    {
        camera.translation.y -= 3800.0 * time.delta_seconds();
    } else if input.just_released(PlayerInput::Crouch)  && input.previous_duration(PlayerInput::Crouch).as_secs_f32() > 1. {
        camera.translation.y += 3800.0 * time.delta_seconds();
    }
}

pub fn check_borders(
    mut player: Query<&mut Transform, With<Player>>, 
    mut camera_query: Query<(&CameraTest, &mut Transform), Without<Player>>,
    window: Query<&Window>
) {
    let mut controller = player.single_mut();
    let window = window.single();

    if controller.translation.y < -window.height() {
        let mut camera = camera_query.single_mut();
        controller.translation.y = window.height();
        camera.1.translation.y = controller.translation.y;
    }
}

pub fn check_terminal_velocity(mut player: Query<&mut Velocity, With<Player>>) {
    let mut controller = player.single_mut();

    if controller.linvel.y < -3000.0 {
        controller.linvel.y = -3000.0;
    }
}

pub fn check_player_collisions(query: Query<&KinematicCharacterControllerOutput, With<Player>>) {
    for controller in query.iter() {
        for collision in controller.collisions.iter() {
            println!("{:?}", collision);
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(move_player)
            .add_system(jump)
            .add_system(check_borders)
            .add_system(check_terminal_velocity)
            .add_system(look_up_down_handle)
            .register_type::<Jump>()
            .register_type::<Health>()
            .register_type::<GravityScale>()
            .register_type::<Speed>()
            .add_plugin(AnimationPlugin);
    }
}

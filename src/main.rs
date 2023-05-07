use animations::{
    player_animations::{change_player_animation, PlayerAnimations},
    sprite_animation::animate_sprite,
};
use bevy::{
    prelude::{
        App, Camera2dBundle, Commands, Component, IntoSystemSetConfig, Plugin, Query, Transform,
        Vec2, With, Without,
    },
    DefaultPlugins,
};
use bevy_ecs_ldtk::{
    prelude::LdtkIntCellAppExt, LdtkPlugin, LdtkSettings, LdtkSystemSet, LevelSelection,
    LevelSpawnBehavior, SetClearColor,
};
use bevy_editor_pls::prelude::EditorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, PhysicsSet, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use entities::{
    blocks::WallBundle,
    player::{
        check_borders, check_player_collisions, check_terminal_velocity, jump, move_player,
        spawn_player, Player, PlayerInput, look_up_down_handle,
    },
};
use leafwing_input_manager::prelude::InputManagerPlugin;
use map::{setup, spawn_map};

pub mod animations;
pub mod entities;
pub mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(EditorPlugin::default())
        .add_plugin(InputManagerPlugin::<PlayerInput>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .insert_resource(LevelSelection::Index(0))
        .add_startup_system(setup)
        .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -2000.),
            ..Default::default()
        })
        .register_ldtk_int_cell::<WallBundle>(1)
        .run()
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_camera, spawn_map))
            .add_system(camera_follow_player)
            .add_plugin(PlayerPlugin);
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
            .add_plugin(AnimationPlugin);
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_player_animation)
            .init_resource::<PlayerAnimations>();
    }
}

#[derive(Component)]
pub struct CameraTest;

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical(250.);
    commands.spawn(camera_bundle).insert(CameraTest);
}

/// This is really rudimentary, but it works for now.
fn camera_follow_player(
    query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&CameraTest, &mut Transform), Without<Player>>,
) {
    let player_transform = query.single();
    let (_, mut camera_transform) = camera_query.single_mut();
    let direction = camera_transform.translation - player_transform.translation;
    camera_transform.translation -= direction * 0.20;
}

struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_system(check_player_collisions);
    }
}


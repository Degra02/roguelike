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
    player::{Player, PlayerInput, PlayerPlugin},
};
use leafwing_input_manager::prelude::InputManagerPlugin;
use map::{ldtk_setup::LdtkLoader, spawn_map};

pub mod animations;
pub mod entities;
pub mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
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
        .configure_set(LdtkSystemSet::ProcessApi.before(PhysicsSet::SyncBackend))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -5000.),
            ..Default::default()
        })
        .register_ldtk_int_cell::<WallBundle>(1)
        .add_plugin(LdtkLoader)
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
    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::FixedVertical(1000.);
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

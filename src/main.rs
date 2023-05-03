use animations::{
    player_animations::{change_player_animation, PlayerAnimations},
    sprite_animation::animate_sprite,
};
use bevy::{
    prelude::{App, Camera2dBundle, Commands, Plugin, IntoSystemSetConfig, Vec2},
    DefaultPlugins,
};
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection, LdtkSystemSet, prelude::LdtkIntCellAppExt};
use bevy_editor_pls::prelude::EditorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin, PhysicsSet, RapierConfiguration},
    render::RapierDebugRenderPlugin,
};
use entities::{
    player::{move_player, spawn_player, PlayerInput, jump, check_borders}, blocks::WallBundle,
};
use leafwing_input_manager::prelude::InputManagerPlugin;
use map::{spawn_map, setup};

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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

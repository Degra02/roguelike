use animations::{
    player_animations::{change_player_animation, PlayerAnimations},
    sprite_animation::animate_sprite,
};
use bevy::{
    prelude::{App, Camera2dBundle, Commands, Plugin},
    DefaultPlugins,
};
use bevy_editor_pls::prelude::EditorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_rapier2d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use entities::{
    hit_box::Grounded,
    player::{double_jump, move_player, spawn_player, Jump, PlayerInput},
};
use leafwing_input_manager::prelude::InputManagerPlugin;
use map::spawn_map;

pub mod animations;
pub mod entities;
pub mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(EditorPlugin::default())
        .add_plugin(InputManagerPlugin::<PlayerInput>::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
        // .add_plugin(InspectableRapierPlugin)
        .add_plugin(RapierDebugRenderPlugin::default())
        .run()
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_camera, spawn_map, spawn_player))
            .add_system(move_player)
            .add_plugin(AnimationPlugin)
            .register_type::<Grounded>()
            .register_type::<Jump>();
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_player_animation)
            .add_system(double_jump)
            .init_resource::<PlayerAnimations>();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

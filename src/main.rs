use animations::{sprite_animation::animate_sprite, player_animations::{change_player_animation, PlayerAnimations}};
use bevy::{
    prelude::{App, Camera2dBundle, Commands, Plugin},
    DefaultPlugins,
};
use bevy_editor_pls::prelude::EditorPlugin;
use entities::{
    player::{move_player, spawn_player, PlayerInput, player_fall, player_jump}
};
use leafwing_input_manager::prelude::InputManagerPlugin;
use map::spawn_map;

pub mod entities;
pub mod animations;
pub mod map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(EditorPlugin::default())
        .add_plugin(InputManagerPlugin::<PlayerInput>::default())
        .run()
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_camera, spawn_map, spawn_player ))
            .add_system(move_player)
            .add_plugin(AnimationPlugin);
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_sprite)
            .add_system(change_player_animation)
            .add_system(player_fall) 
            .add_system(player_jump)
            .init_resource::<PlayerAnimations>(); 
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

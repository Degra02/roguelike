use bevy::{
    prelude::{App, Camera2dBundle, Commands, Plugin},
    DefaultPlugins,
};
use bevy_editor_pls::prelude::EditorPlugin;
use entities::{
    player::{move_player, spawn_player}, animations::{sprite_animation::animate_sprite, player_animations::{change_player_animation, PlayerAnimations}},
};
mod entities;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StartupPlugin)
        .add_plugin(EditorPlugin::default())
        .run()
}

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((spawn_camera, spawn_player))
            .add_systems((animate_sprite, move_player, change_player_animation))
        .init_resource::<PlayerAnimations>();
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

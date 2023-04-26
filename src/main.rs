use bevy::{
    prelude::{App, Camera2dBundle, Commands, Plugin, Query, Res, ResMut, Resource, With},
    time::{Time, Timer},
    DefaultPlugins,
};
use bevy_editor_pls::prelude::EditorPlugin;
use entities::{player::{spawn_player, move_player}, animations::animate_sprite};
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
            .add_systems((animate_sprite, move_player));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

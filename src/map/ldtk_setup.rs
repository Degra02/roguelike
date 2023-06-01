use std::ops::Mul;

use bevy::prelude::{App, AssetServer, Commands, Plugin, Res, Transform, Vec3};
use bevy_ecs_ldtk::LdtkWorldBundle;

use super::generator::Map;

pub struct LdtkLoader;

impl Plugin for LdtkLoader {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ldtk_files = generate_ldtk_files();

    for (ldtk_file, offset) in ldtk_files {
        // Load the LDtk level
        let level_asset = asset_server.load(ldtk_file);

        // Load the level using the LDtk loader
        let level_bundle = LdtkWorldBundle {
            ldtk_handle: level_asset,
            transform: Transform::from_translation(offset.mul(0.7))
                .with_scale(Vec3::new(0.7, 0.7, 1.)), /*.with_scale(Vec3::new(0.25, 0.25, 0.25))*/
            ..Default::default()
        };

        // Spawn the level
        commands.spawn(level_bundle);
    }
}

fn generate_ldtk_files() -> Vec<(String, Vec3)> {
    let map = Map::new(3, 3);
    let mut ldtk_files = Vec::new();

    for (i, _tile) in map.map_tiles.iter().enumerate() {
        // let ldtk_file = match tile {
        //     super::generator::MapTile::Entrance { pos, to } => todo!(),
        //     super::generator::MapTile::Exit { pos, from } => todo!(),
        //     super::generator::MapTile::Path { pos, from, to } => todo!(),
        //     super::generator::MapTile::Empty { pos } => todo!(),
        // };

        let ldtk_file =
            "/home/degra/Coding/Rust/roguelike/map_assets/map/entrances/0.ldtk".to_string();
        // make the level offset in a grid pattern
        ldtk_files.push((
            ldtk_file,
            Vec3::new(
                (i as u32 % map.width) as f32 * 1920.,
                (i as u32 / map.width) as f32 * 1920.,
                0.,
            ),
        ));
    }

    ldtk_files
}

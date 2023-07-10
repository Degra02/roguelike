use std::{fmt::format, ops::Mul};

use bevy::prelude::{App, AssetServer, Commands, Plugin, Res, Transform, Vec3};
use bevy_ecs_ldtk::LdtkWorldBundle;

use super::generator::{Map, MapTile};

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

    for (i, tile) in map.map_tiles.iter().enumerate() {
        let rand = rand::random::<u32>() % 1;
        let ldtk_file = match tile {
            MapTile::Entrance { pos, to } => {
                format!(
                    "/home/degra/Coding/Rust/roguelike/map_assets/map/entrances/{}/{}.ldtk",
                    to, rand
                )
            }
            MapTile::Exit { pos, from } => format!(
                "/home/degra/Coding/Rust/roguelike/map_assets/map/exits/{}/{}.ldtk",
                from, rand,
            ),
            MapTile::Path { pos, from, to } => format!(
                "/home/degra/Coding/Rust/roguelike/map_assets/map/paths/{}-{}/{}.ldtk",
                from, to, rand
            ),
            MapTile::Empty { .. } => {
                let from_dir = match rand::random::<u32>() % 3 {
                    0 => "down",
                    1 => "left",
                    2 => "right",
                    _ => "down",
                };
                let to_dir = match rand::random::<u32>() % 3 {
                    0 => "up",
                    1 => "left",
                    2 => "right",
                    _ => "up",
                };
                format!(
                    "/home/degra/Coding/Rust/roguelike/map_assets/map/{}-{}/{}.ldtk",
                    from_dir, to_dir, rand
                )
            }
        };

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

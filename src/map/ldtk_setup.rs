use bevy::prelude::{Plugin, Vec2, Transform, Vec3, AssetServer, Res, Commands, App};
use bevy_ecs_ldtk::{LdtkLevel, LdtkWorldBundle};

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
            transform: Transform::from_translation(offset),
            ..Default::default()
        };

        // Spawn the level 
        commands.spawn(level_bundle);
    }
}

fn generate_ldtk_files() -> Vec<(String, Vec3)> {
    let map = Map::new(3, 3);  
    let mut ldtk_files = Vec::new();

    for (i, level) in map.map_tiles.iter().enumerate() {
        let ldtk_file = format!("/usr/share/ldtk/extraFiles/samples/AutoLayers_3_Mosaic.ldtk");
        ldtk_files.push((ldtk_file, Vec3::new(i as f32 * 368., -500., 0.)));
    }

    ldtk_files
}

use bevy::{prelude::{Bundle, Component}, sprite::{SpriteBundle}};

use super::collision::CollisionBundle;

#[derive(Bundle)]
pub struct BlockBundle {

    #[bundle]
   sprite: SpriteBundle,

    #[bundle]
    collision: CollisionBundle
}

impl BlockBundle {
    pub fn new(sprite: SpriteBundle, collision: CollisionBundle) -> Self {
        Self {
            sprite,
            collision,
        }
    }
}

#[derive(Debug, Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    _w: Wall,

    #[bundle]
    block_bundle: BlockBundle,
}

impl WallBundle {
    pub fn new(block_bundle: BlockBundle) -> Self {
        Self {
            _w: Wall,
            block_bundle,
        }
    }
}

use bevy::{
    prelude::{Bundle, Component},
    sprite::SpriteBundle,
};
use bevy_ecs_ldtk::LdtkIntCell;

use super::collision::CollisionBundle;

#[derive(Clone, Default, Bundle)]
pub struct BlockBundle {
    #[bundle]
    sprite: SpriteBundle,

    #[bundle]
    collision: CollisionBundle,
}

impl BlockBundle {
    pub fn new(sprite: SpriteBundle, collision: CollisionBundle) -> Self {
        Self { sprite, collision }
    }
}

#[derive(Clone, Default, Debug, Component)]
pub struct Wall;

#[derive(Clone, Default, Bundle, LdtkIntCell)]
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

#[derive(Clone, Default, Debug, Component)]
pub struct Climbable;

#[derive(Bundle, Default, Clone, LdtkIntCell)]
pub struct LadderBundle {
    climbable: Climbable,
}

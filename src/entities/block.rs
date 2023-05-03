use bevy::{prelude::Bundle, sprite::{SpriteSheetBundle, SpriteBundle}};

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

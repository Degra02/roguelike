use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Entity {
    // position: Position,
    // speed: Vec2,
}

#[derive(Component)]
pub struct Position(pub i32, pub i32);

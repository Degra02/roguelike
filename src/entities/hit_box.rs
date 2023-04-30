use bevy::{prelude::{Component, Transform, With, Local, Query}, reflect::Reflect};

use super::player::Player;

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Grounded(pub bool);

pub fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<f32>,
) {
    let (pos,mut on_ground) = player.single_mut();

    let current = (pos.translation.y * 100.).round() == *last;
    if current != on_ground.0 {
        on_ground.0 = current;
    }

    *last = (pos.translation.y * 100.).round();
}

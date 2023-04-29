use bevy::{prelude::{Commands, Vec2, Color}, sprite::{SpriteBundle, Sprite}};

use crate::entities::hit_box::HitBox;


pub fn spawn_map(
    mut commands: Commands
) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(200., 5.)),
            color: Color::WHITE,
            ..Default::default()
        },
        ..Default::default()
    },
        HitBox(Vec2::new(200., 5.))
    ));

}

use bevy::{
    prelude::{Component, Query, Res},
    reflect::Reflect,
    sprite::TextureAtlasSprite,
    time::Time,
};

#[derive(Reflect, Default, Component, Clone, Copy)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
}

#[derive(Reflect, Component, Default, Clone, Copy)]
pub struct FrameTime(pub f32);

pub fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            sprite.index %= animation.len;
            frame_time.0 -= animation.frame_time;
        }
    }
}

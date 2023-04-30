use std::collections::HashMap;

use bevy::{
    prelude::{
        error, AssetServer, Assets, FromWorld, Handle, Input, KeyCode, Query, Res, Resource, Vec2, With,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_rapier2d::prelude::Velocity;

use crate::entities::player::{Jump, Player};

use super::sprite_animation::SpriteAnimation;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Animation {
    Run,
    Idle,
    Jump,
    Fall,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut map = PlayerAnimations {
            map: HashMap::new(),
        };
        let asset_server = world.resource::<AssetServer>();
        let idle_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
            Vec2::splat(32.),
            11,
            1,
            None,
            None,
        );
        let run_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Run (32x32).png"),
            Vec2::splat(32.),
            12,
            1,
            None,
            None,
        );
        let jump_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Jump (32x32).png"),
            Vec2::splat(32.),
            1,
            1,
            None,
            None,
        );
        let fall_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Fall (32x32).png"),
            Vec2::splat(32.),
            1,
            1,
            None,
            None,
        );

        let mut texture_atlas = world.resource_mut::<Assets<TextureAtlas>>();
        map.add(
            Animation::Idle,
            texture_atlas.add(idle_atlas),
            SpriteAnimation {
                len: 11,
                frame_time: 1. / 10.,
            },
        );
        map.add(
            Animation::Run,
            texture_atlas.add(run_atlas),
            SpriteAnimation {
                len: 12,
                frame_time: 1. / 10.,
            },
        );
        map.add(
            Animation::Jump,
            texture_atlas.add(jump_atlas),
            SpriteAnimation {
                len: 1,
                frame_time: 1.,
            },
        );
        map.add(
            Animation::Fall,
            texture_atlas.add(fall_atlas),
            SpriteAnimation {
                len: 1,
                frame_time: 1.,
            },
        );

        map
    }
}

impl PlayerAnimations {
    pub fn add(&mut self, id: Animation, handle: Handle<TextureAtlas>, animation: SpriteAnimation) {
        self.map.insert(id, (handle, animation));
    }

    pub fn get(&self, id: Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
        self.map.get(&id).cloned()
    }
}

pub fn change_player_animation(
    mut player: Query<
        (
            &mut Handle<TextureAtlas>,
            &mut SpriteAnimation,
            &mut TextureAtlasSprite,
            &Jump,
            &Velocity,
        ),
        With<Player>,
    >,
    _input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
) {
    let (mut atlas, mut animation, mut sprite, _jump, velocity) = player.single_mut();
    if velocity.linvel.x < 0. {
        sprite.flip_x = true;
    } else if velocity.linvel.x >= 0. {
        sprite.flip_x = false;
    }

    let set = if velocity.linvel.y > 0.01 {
        Animation::Jump
    } else if velocity.linvel.y < -0.01 {
        Animation::Fall
    } else if velocity.linvel.x != 0. {
        Animation::Run
    } else {
        Animation::Idle
    };

    let Some((new_atlas, new_animation)) = animations.get(set.clone()) else {error!("No Animation {:?} Loaded", set); return;};
    *atlas = new_atlas;
    sprite.index %= new_animation.len;
    *animation = new_animation;
}

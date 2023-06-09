use std::collections::HashMap;

use bevy::{
    prelude::{
        error, AssetServer, Assets, FromWorld, Handle, Input, KeyCode, Query, Res, Resource, Vec2,
        With,
    },
    reflect::Reflect,
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_rapier2d::prelude::Velocity;

use crate::entities::player::Player;

use super::sprite_animation::SpriteAnimation;

#[derive(Reflect, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Animation {
    Run,
    Idle,
    Jump,
    Fall,
    Crouch,
    CrouchWalk,
    LookUp,
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
            asset_server.load("GoldenDude/idle_128x128.png"),
            Vec2::splat(128.),
            1,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );
        let run_atlas = TextureAtlas::from_grid(
            asset_server.load("GoldenDude/run_128x128.png"),
            Vec2::splat(128.),
            8,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );
        let jump_atlas = TextureAtlas::from_grid(
            asset_server.load("GoldenDude/jump_128x128.png"),
            Vec2::splat(128.),
            6,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );
        let fall_atlas = TextureAtlas::from_grid(
            asset_server.load("GoldenDude/fall_128x128.png"),
            Vec2::splat(128.),
            3,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );
        let crouch_atlas = TextureAtlas::from_grid(
            asset_server.load("GoldenDude/crouch_128x128.png"),
            Vec2::splat(128.),
            1,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );
        let crouch_walk_atlas = TextureAtlas::from_grid(
            asset_server.load("GoldenDude/crouch_walk_128x128.png"),
            Vec2::splat(128.),
            7,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );
        let look_up_atlas = TextureAtlas::from_grid(
            asset_server.load("GoldenDude/look_up_128x128.png"),
            Vec2::splat(128.),
            1,
            1,
            None,
            Some(Vec2::new(0., 9.)),
        );

        let mut texture_atlas = world.resource_mut::<Assets<TextureAtlas>>();
        map.add(
            Animation::Idle,
            texture_atlas.add(idle_atlas),
            SpriteAnimation {
                len: 1,
                frame_time: 1. / 10.,
            },
        );
        map.add(
            Animation::Run,
            texture_atlas.add(run_atlas),
            SpriteAnimation {
                len: 8,
                frame_time: 1. / 10.,
            },
        );
        map.add(
            Animation::Jump,
            texture_atlas.add(jump_atlas),
            SpriteAnimation {
                len: 6,
                frame_time: 1. / 4.,
            },
        );
        map.add(
            Animation::Fall,
            texture_atlas.add(fall_atlas),
            SpriteAnimation {
                len: 3,
                frame_time: 1. / 3.,
            },
        );
        map.add(
            Animation::Crouch,
            texture_atlas.add(crouch_atlas),
            SpriteAnimation {
                len: 1,
                frame_time: 1.,
            },
        );
        map.add(
            Animation::CrouchWalk,
            texture_atlas.add(crouch_walk_atlas),
            SpriteAnimation {
                len: 7,
                frame_time: 1. / 6.,
            },
        );
        map.add(
            Animation::LookUp,
            texture_atlas.add(look_up_atlas),
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
            &Velocity,
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
) {
    let (mut atlas, mut animation, mut sprite, velocity) = player.single_mut();
    if velocity.linvel.x < 0. {
        sprite.flip_x = true;
    } else if velocity.linvel.x > 0. {
        sprite.flip_x = false;
    }

    let set = if velocity.linvel.y > 0.01 {
        Animation::Jump
    } else if velocity.linvel.y < -10. {
        Animation::Fall
    } else if input.pressed(KeyCode::S) && velocity.linvel.x != 0. {
        Animation::CrouchWalk
    } else if velocity.linvel.x != 0. {
        Animation::Run
    } else if input.pressed(KeyCode::S) {
        Animation::Crouch
    } else if input.pressed(KeyCode::W) {
        Animation::LookUp
    } else {
        Animation::Idle
    };

    let Some((new_atlas, new_animation)) = animations.get(set.clone()) else {error!("No Animation {:?} Loaded", set); return;};
    *atlas = new_atlas;
    sprite.index %= new_animation.len;
    *animation = new_animation;
}

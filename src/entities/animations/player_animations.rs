use std::collections::HashMap;

use bevy::{
    prelude::{
        AssetServer, Assets, FromWorld, Handle, Input, KeyCode, Query, Res, ResMut, Resource, Vec2,
        With, error,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::entities::player::Player;

use super::sprite_animation::SpriteAnimation;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Animation {
    Run,
    Idle,
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

        let mut texture_atlas = world.resource_mut::<Assets<TextureAtlas>>();
        map.add(Animation::Idle, texture_atlas.add(idle_atlas), SpriteAnimation {len: 11, frame_time: 1./20.});
        map.add(Animation::Run, texture_atlas.add(run_atlas), SpriteAnimation {len: 12, frame_time: 1./20.});

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
        ),
        With<Player>,
    >,
    input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();

    if input.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) {    
    let Some((new_atlas, new_animation)) = animations.get(Animation::Run) else { error!("Failed to find animation: Run"); return;};

        *atlas = new_atlas;
        *animation = new_animation;
        sprite.index = 0;
    }

    if input.any_just_released([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        && !input.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
    {
    let Some((new_atlas, new_animation)) = animations.get(Animation::Idle) else { error!("Failed to find animation: Idle"); return;};

        *atlas = new_atlas;
        *animation = new_animation;
        sprite.index = 0;
    }

    if input.any_just_pressed([KeyCode::A]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::D]) && !input.any_just_pressed([KeyCode::A]) {
        sprite.flip_x = false;
    } else if input.any_just_released([KeyCode::A])
        && !input.any_pressed([KeyCode::A])
        && input.any_pressed([KeyCode::D])
    {
        sprite.flip_x = false;
    }
}

use std::collections::HashMap;

use bevy::{
    prelude::{
        AssetServer, Assets, FromWorld, Handle, Input, KeyCode, Query, Res, Resource, Vec2,
        With, error, Transform,
    },
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::entities::player::{Player, Jump};

use super::sprite_animation::SpriteAnimation;

#[derive(Debug, Hash, PartialEq, Eq)]
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
        map.add(Animation::Idle, texture_atlas.add(idle_atlas), SpriteAnimation {len: 11, frame_time: 1./20.});
        map.add(Animation::Run, texture_atlas.add(run_atlas), SpriteAnimation {len: 12, frame_time: 1./20.});
        map.add(Animation::Run, texture_atlas.add(jump_atlas), SpriteAnimation {len: 1, frame_time: 1.});
        map.add(Animation::Run, texture_atlas.add(fall_atlas), SpriteAnimation {len: 1, frame_time: 1.});

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
    player_jump: Query<(&Transform, Option<&Jump>), With<Player>>,
    input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();
    let (pos, jump) = player_jump.single();

    if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right])
    && !input.any_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = false;
    } else if input.any_just_released([KeyCode::A, KeyCode::Left])
    && !input.any_pressed([KeyCode::A, KeyCode::Left])
    && input.any_pressed([KeyCode::D, KeyCode::Right]) {
        sprite.flip_x = false;
    }

    //Jumping if jump
    if jump.is_some() {
        let Some((new_atlas, new_animaiton)) = animations.get(Animation::Jump) else {error!("No Animation Jump Loaded"); return;};
        *atlas = new_atlas;
        *animation = new_animaiton;
        sprite.index = 0;
        return;
    //Falling if Y > 0.0
    } else if pos.translation.y > 0.0 {
        let Some((new_atlas, new_animaiton)) = animations.get(Animation::Fall) else {error!("No Animation Fall Loaded"); return;};
        *atlas = new_atlas;
        *animation = new_animaiton;
        sprite.index = 0;
        return;
    }

    if input.any_just_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        let Some((new_atlas, new_animaiton)) = animations.get(Animation::Run) else {error!("No Animation Run Loaded"); return;};
        *atlas = new_atlas;
        *animation = new_animaiton;
        sprite.index = 0;
    }
    //if no move keys pressed set idel animtaion
    if input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
    && !input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        let Some((new_atlas, new_animaiton)) = animations.get(Animation::Idle) else {error!("No Animation Idle Loaded"); return;};
        *atlas = new_atlas;
        *animation = new_animaiton;
        sprite.index = 0;
    }

}

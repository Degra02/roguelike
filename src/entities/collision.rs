use bevy::{
    prelude::{Bundle, Resource},
    reflect::Reflect,
};
use bevy_ecs_ldtk::{EntityInstance, LdtkIntCell};
use bevy_rapier2d::prelude::{Collider, GravityScale, LockedAxes, RigidBody, Velocity};

#[derive(Bundle, Clone, Debug, LdtkIntCell, Resource)]
pub struct CollisionBundle {
    rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes,
    velocity: Velocity,
    gravity_scale: GravityScale,
}

impl Default for CollisionBundle {
    fn default() -> Self {
        Self {
            rigid_body: RigidBody::Fixed,
            collider: Collider::cuboid(64., 64.),
            locked_axes: Default::default(),
            velocity: Default::default(),
            gravity_scale: GravityScale(0.0),
        }
    }
}

impl CollisionBundle {
    pub fn new(
        rigid_body: RigidBody,
        collider: Collider,
        locked_axes: LockedAxes,
        velocity: Velocity,
        gravity_scale: GravityScale,
    ) -> Self {
        Self {
            rigid_body,
            collider,
            locked_axes,
            velocity,
            gravity_scale,
        }
    }
}

impl From<&EntityInstance> for CollisionBundle {
    fn from(entity_instance: &EntityInstance) -> Self {
        let _rotation_constraints = LockedAxes::ROTATION_LOCKED;

        match entity_instance.identifier.as_ref() {
            "Player" => {
                todo!()
            }
            "Mob" => {
                todo!()
            }

            _ => {
                todo!()
            }
        }
    }
}

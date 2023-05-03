use bevy::prelude::Bundle;
use bevy_ecs_ldtk::{LdtkIntCell, EntityInstance};
use bevy_rapier2d::prelude::{RigidBody, Collider, LockedAxes, Velocity};

#[derive(Bundle, Clone, Default, Debug, LdtkIntCell)]
pub struct CollisionBundle {
   rigid_body: RigidBody,
    collider: Collider,
    locked_axes: LockedAxes, 
    velocity: Velocity,
} 

impl CollisionBundle {
    pub fn new(rigid_body: RigidBody, collider: Collider, locked_axes: LockedAxes, velocity: Velocity) -> Self {
        Self {
            rigid_body,
            collider,
            locked_axes,
            velocity
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

            _ => { todo!()}
        }
    }
}

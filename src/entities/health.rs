use bevy::{prelude::Component, reflect::Reflect};

#[derive(Reflect, Component)]
pub struct Health(i32);

impl Health {
    pub fn new(hp: i32) -> Self {
        Self(hp)
    }

    pub fn hit(&mut self) -> Option<i32> {
        self.0 -= 1;

        if self.0 == 0 {
            None
        } else {
            Some(self.0)
        }
    }
}

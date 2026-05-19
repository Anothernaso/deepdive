use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Transform)]
#[reflect(Component)]
pub struct Body {
    density: f32
}

impl Body {
    pub fn new(density: f32) -> Self {
        Self { density }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self {
            density: 7850.,
        }
    }
}

#[derive(Component, Reflect)]
#[require(Body)]
#[reflect(Component)]
pub struct DefaultBody;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Buoyancy;

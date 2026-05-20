use bevy::prelude::*;

use deepdive_physics::{Body, DefaultBody};

#[derive(Default, Component, Reflect)]
#[require(DefaultBody)]
#[reflect(Component)]
pub struct Human;

pub fn human() -> impl Bundle {
    let mass = 0.5 * 1.8;

    (
        Human,
        Body::new(mass, mass)
    )
}

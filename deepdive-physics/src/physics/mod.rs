use bevy::prelude::*;
use crate::body::{Body, DefaultBody};


pub fn process_physics(
    commands: Commands,
    buoyant: Query<(&mut Transform, &Body), With<DefaultBody>>
) {
    buoyant.par_iter().for_each(|(mut transform, body)| {});
}

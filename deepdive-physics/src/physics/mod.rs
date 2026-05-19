use bevy::prelude::*;
use crate::{body::{Body, DefaultBody, Buoyancy}, water_setup::WaterSetup};


pub fn process_physics(
    mut buoyant: Query<(&mut Transform, &Body), (With<DefaultBody>, With<Buoyancy>)>,
    water_setup: Res<WaterSetup>,
    time: Res<Time>
) {
    buoyant.par_iter_mut().for_each(
        |(mut transform, body)| {
            let density_ratio = body.get_density_kgpm2() / water_setup.density_kgpm2;

            transform.translation.y += (density_ratio - 1.) * time.delta_secs();
        }
    );
}

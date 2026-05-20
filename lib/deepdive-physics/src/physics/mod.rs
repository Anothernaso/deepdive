use bevy::prelude::*;

use crate::{body::{Body, DefaultBody, Buoyancy}, water_setup::WaterSetup};

use deepdive_state::IsPaused;

pub fn process_physics(
    mut buoyant: Query<(&mut Transform, &Body), (With<DefaultBody>, With<Buoyancy>)>,
    water_setup: Res<WaterSetup>,
    is_paused: Res<State<IsPaused>>,
    time: Res<Time>
) {
    if *is_paused.get() != IsPaused::Running {
        return;
    }

    buoyant.par_iter_mut().for_each(
        |(mut transform, body)| {
            let density_ratio = body.get_density_kgpm2() / water_setup.density_kgpm2;

            transform.translation.y += (density_ratio - 1.) * time.delta_secs();
        }
    );
}

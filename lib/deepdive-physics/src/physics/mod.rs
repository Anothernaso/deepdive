// Copyright (C) 2026 Anatnaso
//
// This file is part of Project Deep Dive.
//
// Project Deep Dive is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation, either
// version 3 of the License, or any later version.
//
// Project Deep Dive is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with
// Project Deep Dive. If not, see <https://www.gnu.org/licenses/>.
//

use bevy::prelude::*;

use crate::{body::{Body, DefaultBody, Buoyant}, water_setup::WaterSetup};

use deepdive_state::IsPaused;

pub fn process_physics(
    mut buoyant: Query<(&mut Transform, &Body), (With<DefaultBody>, With<Buoyant>)>,
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

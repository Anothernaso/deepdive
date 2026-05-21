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

use crate::{body::{PhysicsBody, DefaultSimulatedBody, SubAquaticBody}, water_setup::WaterSetup};

use deepdive_state::IsPaused;

pub fn simulate_physics(
    mut sub_aquatic: Query<(&mut Transform, &PhysicsBody), (With<DefaultSimulatedBody>, With<SubAquaticBody>)>,
    water_setup: Res<WaterSetup>,
    is_paused: Res<State<IsPaused>>,
    time: Res<Time>
) {
    if *is_paused.get() != IsPaused::Running {
        return;
    }

    sub_aquatic.par_iter_mut().for_each(
        |(mut transform, body)| {
            let density_ratio = body.get_density_dagpcm2() / water_setup.density_dagpcm2;

            let vel_delta = (density_ratio - 1.) * 100. * time.delta_secs();

            transform.translation.y += vel_delta;
        }
    );
}

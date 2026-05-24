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

use super::{
    body::{PhysicsBody, SubAquaticBody},
    message::BodyUpdate,
    water_setup::WaterSetup,
};

use deepdive_state::IsPaused;

pub fn simulate_buoyancy(
    sub_aquatic: Query<(Entity, &PhysicsBody), With<SubAquaticBody>>,
    water_setup: Res<WaterSetup>,
    mut writer: MessageWriter<BodyUpdate>,
    is_paused: Res<State<IsPaused>>,
    time: Res<Time>,
) {
    if *is_paused.get() != IsPaused::Running {
        return;
    }

    let mut messages = Vec::<BodyUpdate>::new();

    sub_aquatic.iter().for_each(|(entity, body)| {
        let density_ratio = body.get_density_dagpcm2() / water_setup.density_dagpcm2;
        let vel_delta = (density_ratio - 1.) * 100. * time.delta_secs();

        messages.push(BodyUpdate::new(entity, Vec2::new(0., vel_delta)));
    });

    writer.write_batch(messages);
}

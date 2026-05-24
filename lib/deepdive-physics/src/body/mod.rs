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
    misc::{Area, Density, Mass, Size},
    setup::WaterSetup,
};

#[derive(Debug, Default, Component, Reflect)]
#[require(Transform, Size, Mass, Area, Density)]
#[reflect(Component)]
pub struct PhysicsBody;

#[derive(Debug, Default, Component, Reflect)]
#[require(PhysicsBody)]
#[reflect(Component)]
pub struct SubAquaticBody {
    pub is_submerged: bool,
}

pub fn update_subaquatic_body(
    mut query: Query<
        (&mut SubAquaticBody, &Transform, &Size),
        Or<(Added<SubAquaticBody>, Changed<Transform>, Changed<Size>)>,
    >,
    water_setup: Res<WaterSetup>,
) {
    query.iter_mut().for_each(|(mut body, transform, size)| {
        let lower_bound = transform.translation.y - size.0.y / 2.;

        body.is_submerged = lower_bound < water_setup.surface_height;
    });
}

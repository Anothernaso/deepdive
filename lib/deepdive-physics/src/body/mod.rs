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
    misc::{Density, Size},
    setup::WaterSetup,
};

#[derive(Debug, Default, Component, Reflect)]
#[require(Transform)]
#[reflect(Component)]
pub struct PhysicsBody;

#[derive(Debug, Default, Component, Reflect)]
#[require(Transform, Size)]
#[reflect(Component)]
pub struct Submerged(pub bool);

#[derive(Debug, Default, Component, Reflect)]
#[require(Submerged, Density, PhysicsBody)]
#[reflect(Component)]
pub struct Buoyant;

#[derive(Debug, Default, Component, Reflect)]
#[require(PhysicsBody)]
#[reflect(Component)]
pub struct Gravitational;

pub fn update_submerged(
    mut query: Query<
        (&mut Submerged, &Transform, &Size),
        Or<(Added<Submerged>, Changed<Transform>, Changed<Size>)>,
    >,
    water_setup: Res<WaterSetup>,
) {
    query
        .iter_mut()
        .for_each(|(mut submerged, transform, size)| {
            let lower_bound = transform.translation.y - size.0.y / 2.;

            submerged.0 = lower_bound < water_setup.surface_height;
        });
}

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

/// Unit: Centimeters
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Size(pub Vec2);

/// Unit: Decagrams
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Mass(pub f32);

/// Unit: Square centimeters
#[derive(Debug, Default, Component, Reflect)]
#[require(Size)]
#[reflect(Component)]
pub struct Area(pub f32);

/// Unit: Decagrams per square centimeter
#[derive(Debug, Default, Component, Reflect)]
#[require(Area, Mass)]
#[reflect(Component)]
pub struct Density(pub f32);

pub fn update_area(mut query: Query<(&mut Area, &Size), Or<(Added<Area>, Changed<Size>)>>) {
    query.iter_mut().for_each(|(mut area, size)| {
        area.0 = size.0.x * size.0.y;
    });
}

pub fn update_density(
    mut query: Query<
        (&mut Density, &Mass, &Area),
        Or<(Added<Density>, Changed<Mass>, Changed<Area>)>,
    >,
) {
    query.iter_mut().for_each(|(mut density, mass, area)| {
        density.0 = mass.0 / area.0;
    });
}

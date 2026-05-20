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

#[derive(Component, Reflect)]
#[require(Transform)]
#[reflect(Component)]
pub struct Body {
    area_m2: f32,
    mass_kg: f32,

    density_kgpm2: f32
}

impl Body {
    pub fn new(area_m2: f32, mass_kg: f32) -> Self {
        Self { area_m2, mass_kg, density_kgpm2: mass_kg / area_m2 }
    }

    pub fn get_area_m2(&self) -> f32 {
        self.area_m2
    }
    pub fn set_area_m2(&mut self, area_m2: f32) {
        self.area_m2 = area_m2;
        self.density_kgpm2 = self.mass_kg / self.area_m2;
    }

    pub fn get_mass_kg(&self) -> f32 {
        self.mass_kg
    }
    pub fn set_mass_kg(&mut self, mass_kg: f32) {
        self.mass_kg = mass_kg;
        self.density_kgpm2 = self.mass_kg / self.area_m2;
    }

    pub fn get_density_kgpm2(&self) -> f32 {
        self.density_kgpm2
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

#[derive(Default, Component, Reflect)]
#[require(Body)]
#[reflect(Component)]
pub struct DefaultBody;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Buoyant;

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
pub struct PhysicsBody {
    area_cm2: f32,
    mass_dag: f32,

    density_dagpcm2: f32
}

impl PhysicsBody {
    pub fn new(area_cm2: f32, mass_dag: f32) -> Self {
        Self { area_cm2, mass_dag, density_dagpcm2: mass_dag / area_cm2 }
    }

    pub fn get_area_cm2(&self) -> f32 {
        self.area_cm2
    }
    pub fn set_area_cm2(&mut self, area_cm2: f32) {
        self.area_cm2 = area_cm2;
        self.density_dagpcm2 = self.mass_dag / self.area_cm2;
    }

    pub fn get_mass_dag(&self) -> f32 {
        self.mass_dag
    }
    pub fn set_mass_dag(&mut self, mass_dag: f32) {
        self.mass_dag = mass_dag;
        self.density_dagpcm2 = self.mass_dag / self.area_cm2;
    }

    pub fn get_density_dagpcm2(&self) -> f32 {
        self.density_dagpcm2
    }
}

impl Default for PhysicsBody {
    fn default() -> Self {
        Self::new(100., 100.)
    }
}

#[derive(Default, Component, Reflect)]
#[require(PhysicsBody)]
#[reflect(Component)]
pub struct SubAquaticBody;

#[derive(Default, Component, Reflect)]
#[require(PhysicsBody)]
#[reflect(Component)]
pub struct DefaultSimulatedBody;

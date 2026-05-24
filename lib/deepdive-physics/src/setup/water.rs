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

#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct WaterSetup {
    /// Unit: Centimeters
    pub surface_height: f32,

    /// Unit: Decagrams per square centimeter
    pub density: f32,
}

impl WaterSetup {
    pub fn new(surface_height: f32, density: f32) -> Self {
        Self {
            surface_height,
            density,
        }
    }
}

impl Default for WaterSetup {
    fn default() -> Self {
        Self {
            surface_height: 0.,
            density: 1.,
        }
    }
}

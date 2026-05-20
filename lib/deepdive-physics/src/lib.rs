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

mod water_setup;
mod body;
mod physics;

pub use water_setup::WaterSetup;
pub use body::{Body, DefaultBody, Buoyant};

use bevy::prelude::*;

use physics::process_physics;


#[derive(Default)]
pub struct DeepDivePhysicsPlugin {
    water_setup: WaterSetup,
}

impl DeepDivePhysicsPlugin {
    pub fn new(water_setup: WaterSetup) -> Self {
        Self { water_setup }
    }
}

impl Plugin for DeepDivePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WaterSetup>();

        app.register_type::<Body>();
        app.register_type::<DefaultBody>();
        app.register_type::<Buoyant>();

        app.insert_resource(self.water_setup.clone());

        app.add_systems(FixedUpdate, process_physics);
    }
}

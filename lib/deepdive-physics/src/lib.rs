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

mod body;
mod message;
mod misc;
mod physics;
mod water_setup;

pub use body::{PhysicsBody, SubAquaticBody};
pub use message::BodyUpdate;
pub use misc::{Area, Density, Mass, Size};
pub use water_setup::WaterSetup;

use bevy::prelude::*;

use message::body_update;
use misc::{update_area, update_density};
use physics::simulate_buoyancy;

use deepdive_state::IsPaused;

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

        app.register_type::<BodyUpdate>();

        app.register_type::<PhysicsBody>();
        app.register_type::<SubAquaticBody>();

        app.register_type::<Size>();
        app.register_type::<Mass>();
        app.register_type::<Area>();
        app.register_type::<Density>();

        app.insert_resource(self.water_setup.clone());

        app.add_message::<BodyUpdate>();

        app.add_systems(FixedPreUpdate, (update_area, update_density).chain());
        app.add_systems(
            FixedUpdate,
            simulate_buoyancy.run_if(in_state(IsPaused::Running)),
        );
        app.add_systems(FixedPostUpdate, body_update);
    }
}

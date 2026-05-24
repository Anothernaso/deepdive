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
mod setup;

pub use body::{PhysicsBody, SubAquaticBody};
pub use message::BodyUpdate;
pub use misc::{Area, Density, Mass, Size};
pub use setup::{PhysicsSetup, WaterSetup};

use bevy::prelude::*;

use body::update_subaquatic_body;
use message::body_update;
use misc::{update_area, update_density};
use physics::{apply_buoyancy, apply_gravity};

use deepdive_state::IsPaused;

#[derive(Default)]
pub struct DeepDivePhysicsPlugin {
    physics_setup: PhysicsSetup,
    water_setup: WaterSetup,
}

impl DeepDivePhysicsPlugin {
    pub fn new(physics_setup: PhysicsSetup, water_setup: WaterSetup) -> Self {
        Self {
            physics_setup,
            water_setup,
        }
    }
}

impl Plugin for DeepDivePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PhysicsSetup>();
        app.register_type::<WaterSetup>();

        app.register_type::<BodyUpdate>();

        app.register_type::<PhysicsBody>();
        app.register_type::<SubAquaticBody>();

        app.register_type::<Size>();
        app.register_type::<Mass>();
        app.register_type::<Area>();
        app.register_type::<Density>();

        app.insert_resource(self.physics_setup.clone());
        app.insert_resource(self.water_setup.clone());

        app.add_message::<BodyUpdate>();

        app.add_systems(
            FixedPreUpdate,
            (update_area, update_density, update_subaquatic_body).chain(),
        );
        app.add_systems(
            FixedUpdate,
            (apply_buoyancy, apply_gravity).run_if(in_state(IsPaused::Running)),
        );
        app.add_systems(FixedPostUpdate, body_update);
    }
}

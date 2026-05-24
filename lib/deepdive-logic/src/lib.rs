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

mod camera;
mod controller;
mod pawn;

pub use camera::MainCamera;
pub use controller::{Controller, PlayerController, player_controller};
pub use pawn::{HumanPawn, Pawn, human};

use bevy::prelude::*;

use camera::camera_setup;
use controller::update_player;

use deepdive_state::IsPaused;

pub struct DeepDiveLogicPlugin;

impl Plugin for DeepDiveLogicPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>();

        app.register_type::<Controller>();
        app.register_type::<PlayerController>();

        app.register_type::<Pawn>();
        app.register_type::<HumanPawn>();

        app.add_systems(Startup, (camera_setup, setup));
        app.add_systems(
            FixedUpdate,
            update_player.run_if(in_state(IsPaused::Running)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(human()).with_children(|p| {
        p.spawn(player_controller(p.target_entity()));
    });
}

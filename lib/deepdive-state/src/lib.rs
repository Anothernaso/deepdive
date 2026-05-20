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

mod state;

pub use state::{AppState, IsPaused};

use bevy::prelude::*;

pub struct DeepDiveStatePlugin;

impl Plugin for DeepDiveStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppState>();
        app.register_type::<IsPaused>();

        app.init_state::<AppState>();
        app.add_sub_state::<IsPaused>();
    }
}

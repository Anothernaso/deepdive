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

use super::state::IsPaused;

pub fn update_toggle(
    state: Res<State<IsPaused>>,
    mut next: ResMut<NextState<IsPaused>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match *state.get() {
            IsPaused::Running => next.set(IsPaused::Paused),
            IsPaused::Paused => next.set(IsPaused::Running),
        };
    }
}

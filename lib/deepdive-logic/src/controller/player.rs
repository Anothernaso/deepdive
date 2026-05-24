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

use super::{super::pawn::Pawn, Controller};

use deepdive_physics::BodyUpdate;
use deepdive_state::IsPaused;

const SPEED: f32 = 3.3;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerController;

pub fn player_controller(pawn: Entity) -> impl Bundle {
    (Controller::new(pawn), PlayerController)
}

pub fn update_player(
    players: Query<&Controller, With<PlayerController>>,
    pawns: Query<Entity, With<Pawn>>,
    mut writer: MessageWriter<BodyUpdate>,
    input: Res<ButtonInput<KeyCode>>,
    state: Res<State<IsPaused>>,
    time: Res<Time>,
) {
    if *state.get() != IsPaused::Running {
        return;
    }

    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) {
        dir.y += 1.;
    }
    if input.pressed(KeyCode::KeyA) {
        dir.x -= 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        dir.y -= 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        dir.x += 1.;
    }

    let dir = dir.normalize_or_zero();
    let vel = dir * 100. * SPEED * time.delta_secs();

    let mut messages = Vec::<BodyUpdate>::new();

    players.iter().for_each(|controller| {
        let Ok(pawn) = pawns.get(controller.pawn) else {
            return;
        };

        messages.push(BodyUpdate::new(pawn, vel));
    });

    writer.write_batch(messages);
}

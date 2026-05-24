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

use crate::{body::PhysicsBody, message::BodyUpdate, setup::PhysicsSetup};

pub fn apply_gravity(
    query: Query<Entity, With<PhysicsBody>>,
    gravity: Res<PhysicsSetup>,
    mut writer: MessageWriter<BodyUpdate>,
    time: Res<Time>,
) {
    let mut messages = Vec::<BodyUpdate>::new();

    query.iter().for_each(|entity| {
        let vel_delta = -(gravity.gravity_scale * 100. * time.delta_secs());

        let message = BodyUpdate::new(entity, Vec2::new(0., vel_delta));

        messages.push(message);
    });

    writer.write_batch(messages);
}

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

use super::Pawn;
use deepdive_physics::{PhysicsBody, SubAquaticBody};

#[derive(Component, Reflect)]
#[require(Pawn, SubAquaticBody)]
#[reflect(Component)]
pub struct HumanPawn;

pub fn human() -> impl Bundle {
    let size = Vec2::new(50., 180.);
    let weight = 8000.;

    (
        HumanPawn,
        PhysicsBody::new(size.x * size.y, weight),
        Sprite::from_color(Color::WHITE, size),
    )
}

use std::sync::{Arc, RwLock};

use bevy::prelude::*;

use super::body::PhysicsBody;

use deepdive_state::IsPaused;

#[derive(Message, Reflect)]
pub struct BodyUpdate {
    body: Entity,
    delta: Vec2,
}

impl BodyUpdate {
    pub fn new(body: Entity, delta: Vec2) -> Self {
        Self { body, delta }
    }
}

pub fn body_update(
    bodies: Query<&mut Transform, With<PhysicsBody>>,
    mut messages: MessageReader<BodyUpdate>,
    is_paused: Res<State<IsPaused>>,
) {
    if *is_paused.get() != IsPaused::Running {
        return;
    }

    let bodies = Arc::new(RwLock::new(bodies));

    messages.par_read().for_each(|message| {
        let Ok(mut bodies) = bodies.write() else {
            return;
        };

        let Ok(mut body) = bodies.get_mut(message.body) else {
            return;
        };

        body.translation += message.delta.extend(0.);
    });
}

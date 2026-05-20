mod water_setup;
mod body;
mod physics;

pub use water_setup::WaterSetup;
pub use body::{Body, DefaultBody, Buoyancy};

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
        app.register_type::<Buoyancy>();

        app.insert_resource(self.water_setup.clone());

        app.add_systems(FixedUpdate, process_physics);
    }
}

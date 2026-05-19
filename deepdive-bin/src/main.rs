use bevy::prelude::*;

use bevy_seedling::prelude::*;
use avian2d::prelude::*;

use deepdive_physics::DeepDivePhysicsPlugin;
use deepdive_state::DeepDiveStatePlugin;


pub struct DeepDiveMasterPlugin;

impl Plugin for DeepDiveMasterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
                DeepDiveStatePlugin,
                DeepDivePhysicsPlugin::default(),
            )
        );
    }
}

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins,
                SeedlingPlugin::default(),
                PhysicsPlugins::default(),
                DeepDiveMasterPlugin,
            )
        )
        .run();
}

mod camera;

pub use camera::MainCamera;

use bevy::prelude::*;

use camera::camera_setup;

pub struct DeepDiveLogicPlugin;

impl Plugin for DeepDiveLogicPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>();

        app.add_systems(Startup, camera_setup);
    }
}

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

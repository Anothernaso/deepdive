use bevy::prelude::*;

#[derive(Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct WaterSetup {
    pub surface_height: f32,
    pub density: f32,
}

impl WaterSetup {
    pub fn new(surface_height: f32, density: f32) -> Self {
        Self { surface_height, density }
    }
}

impl Default for WaterSetup {
    fn default() -> Self {
        Self {
            surface_height: 0.,
            density: 999.972,
        }
    }
}

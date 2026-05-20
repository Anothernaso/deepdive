use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Camera2d)]
#[reflect(Component)]
pub struct MainCamera;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn(MainCamera);
}

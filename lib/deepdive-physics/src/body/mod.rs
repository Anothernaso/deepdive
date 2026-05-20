use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(Transform)]
#[reflect(Component)]
pub struct Body {
    area_m2: f32,
    mass_kg: f32,

    density_kgpm2: f32
}

impl Body {
    pub fn new(area_m2: f32, mass_kg: f32) -> Self {
        Self { area_m2, mass_kg, density_kgpm2: mass_kg / area_m2 }
    }

    pub fn get_area_m2(&self) -> f32 {
        self.area_m2
    }
    pub fn set_area_m2(&mut self, area_m2: f32) {
        self.area_m2 = area_m2;
        self.density_kgpm2 = self.mass_kg / self.area_m2;
    }

    pub fn get_mass_kg(&self) -> f32 {
        self.mass_kg
    }
    pub fn set_mass_kg(&mut self, mass_kg: f32) {
        self.mass_kg = mass_kg;
        self.density_kgpm2 = self.mass_kg / self.area_m2;
    }

    pub fn get_density_kgpm2(&self) -> f32 {
        self.density_kgpm2
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

#[derive(Component, Reflect)]
#[require(Body)]
#[reflect(Component)]
pub struct DefaultBody;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Buoyancy;

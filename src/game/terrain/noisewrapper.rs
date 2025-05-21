use bevy::prelude::*;
use noise::Perlin;
use std::ops::Deref;

#[derive(Resource)]
pub struct NoisePerlin(pub Perlin);

impl Deref for NoisePerlin {
    type Target = Perlin;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
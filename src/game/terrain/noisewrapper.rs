use bevy::prelude::*;
use noise::Perlin;

#[derive(Resource)]
pub struct NoisePerlin(pub Perlin);
/* The noise.rs file will be to create different noise functions to be used in the enviornment.
Various builiding heights and to keep this project modular
For right now it is very basic, I want to create a more complex noise function in the future
to make it easier to control from this file. */

use noise::{NoiseFn, Perlin};

// generates a Perlin noise value for a given x, y coordinate.
// This can be used for random heights, terrain features, or other variabilities.
pub fn generate_perlin_noise(x: f64, y: f64) -> f64 {
    let perlin = Perlin::new();
    perlin.get([x, y])
}

//scaled Perlin noise for building height generation
// This will give a value that is scaled based on the coordinate inputs.
pub fn get_scaled_building_height(x: f64, y: f64, scale: f64) -> f64 {
    let perlin_value = generate_perlin_noise(x, y);
    perlin_value * scale
}

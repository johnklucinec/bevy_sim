/// Author: Brant Cass (@brantcass)

pub mod biome;
pub mod road;
pub mod roadspline;
pub mod randomroad;
mod systems;

//pub use randomroad::spawn_grid_roads;
pub use roadspline::Spline;
pub use road::{Segment, spawn_single_road};

use crate::AppState;
use bevy::prelude::*;
use systems::spawn_biome_on_enter; // Import systems

///Plugin for generating the biome (trees, grass, roads)
pub struct BiomePlugin;

impl Plugin for BiomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_biome_on_enter);
    }
}

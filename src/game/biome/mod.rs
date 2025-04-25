pub mod biome;
pub mod road;
pub mod rand_objects;
mod systems;

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

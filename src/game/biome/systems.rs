use bevy::prelude::*;
use crate::game::biome::road::spawn_single_road;
use crate::game::biome::road::spawn_grid_roads;
use crate::game::biome::biome::setup_terrain;

//Spawns in roads
pub fn spawn_biome_on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_terrain(&mut commands, &mut meshes, &mut materials);
    //spawn_grid_roads(&mut commands, &mut meshes, &mut materials, 5, 5, 10.0);
    spawn_single_road(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        Vec3::new(-500.0, 0.0, 0.0),
        Vec3::new(500.0, 0.0, 0.0),
    );
}

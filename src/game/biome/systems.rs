/// Author: Brant Cass (@brantcass)
use crate::game::biome::biome::setup_terrain;
//use crate::game::biome::randomroad::spawn_grid_roads;
use crate::game::biome::road::spawn_single_road;
use crate::game::biome::Spline;
use bevy::prelude::*;

pub fn spawn_biome_on_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //Build the road and grab its segments
    let road_segments = spawn_single_road(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 0.0, -500.0),
        Vec3::new(0.0, 0.0, 500.0),
    );

    // If switch to the grid version, just comment out code above
    // and uncomment this:
    // let road_segments = spawn_grid_roads(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     5, 5, 10.0,
    // );

    //expose as resource
    commands.insert_resource(Spline::from_segments(&road_segments));

    setup_terrain(&mut commands, &mut meshes, &mut materials);
}

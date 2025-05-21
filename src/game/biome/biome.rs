/// Author: Brant Cass (@brantcass)
/*biome.rs is to create the enviornment around the vehicle */

use bevy::prelude::*;
use bevy::render::mesh::Mesh;


pub fn setup_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    //Sky
    //build a large sphere
    let sky_tex: Handle<Image> =
        asset_server.load("3dmodels/evening_road_01_puresky_4k.hdr");

    let sky_mat = materials.add(StandardMaterial {
        base_color_texture: Some(sky_tex),
        unlit: true,
        ..Default::default()
    });

    let sky_mesh = meshes.add(Sphere::new(1000.0).mesh());

    commands.spawn((
        Mesh3d(sky_mesh),
        MeshMaterial3d(sky_mat),
        Transform::from_scale(Vec3::new(-1.0, 1.0, 1.0)),
    ));
}



/* The terrain file for right now will create a basic road, I was aiming for a simple neighborhood
just for right now. Basically using boxes to form it 

The code is using boxes meshes for everything right now to create a intersection as well as the 
white strip down the middle
*/

use bevy::prelude::*;
use crate::style::BiomeStyle; 

pub fn setup_terrain(commands: &mut Commands) {
    //create BiomeStyle instance to get color schemes
    let biome_style = BiomeStyle::default();

    //horizontal road
    commands.spawn_bundle(PbrBundle {
        mesh: bevy::prelude::shape::Box::new(100.0, 1.0, 5.0),
        material: StandardMaterial {
            base_color: biome_style.road_color, //dark gray
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 15.0),
            scale: Vec3::new(50.0, 0.1, 5.0),
            ..Default::default()
        },
        ..Default::default()
    });

    //vertical road
    commands.spawn_bundle(PbrBundle {
        mesh: bevy::prelude::shape::Box::default(),
        material: StandardMaterial {
            base_color: biome_style.road_color, // dark gray
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(15.0, 0.0, 0.0),
            scale: Vec3::new(5.0, 0.1, 50.0),
            ..Default::default()
        },
        ..Default::default()
    });

    //adding a white strip down the middle of the horizontal road
    commands.spawn_bundle(PbrBundle {
        mesh: bevy::prelude::shape::Box::default(),
        material: StandardMaterial {
            base_color: biome_style.strip_color, //white
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.1, 15.0),
            scale: Vec3::new(50.0, 0.1, 5.0),
            ..Default::default()
        },
        ..Default::default()
    });

    //adding a white strip down the middle of the vertical road
    commands.spawn_bundle(PbrBundle {
        mesh: bevy::prelude::shape::Box::default(),
        material: StandardMaterial {
            base_color: biome_style.strip_color, // white
            ..Default::default()
        },
        //positioning the strip
        transform: Transform {
            translation: Vec3::new(15.0, 0.05, 0.0),
            scale: Vec3::new(5.0, 0.05, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
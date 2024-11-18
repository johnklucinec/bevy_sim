use bevy::prelude::*;

#[derive(Component)]
pub struct GridLocation {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Movable;

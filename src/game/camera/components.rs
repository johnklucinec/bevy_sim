use bevy::prelude::*;
use std::process::Child;

#[derive(Component)]
pub struct SecondaryCamera;

#[derive(Component)]
pub struct CarFollowCamera;

#[derive(Resource)]
pub struct PythonProcess(pub Option<Child>);

#[derive(Resource)]
pub struct SecondaryWindow(pub Entity);

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SecondaryCameraState {
    #[default]
    Hidden, // State for the main menu
    Visible, // State for when the game is running
}

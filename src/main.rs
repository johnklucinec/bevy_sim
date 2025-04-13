use bevy::prelude::*;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<CameraState>()
        // Game Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Game Systems
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(Update, (change_camera_state, exit_game))
        // Camera Systems
        .add_systems(FixedUpdate, update_car_camera // MUST be FixedUpdate to prevent jitter
            .run_if(in_state(AppState::Game))
            .run_if(in_state(CameraState::CarCam)))
        .add_systems(FixedUpdate, move_camera
            .run_if(in_state(AppState::Game))
            .run_if(in_state(CameraState::FreeCam))
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu, // State for the main menu
    Game,     // State for when the game is running
    GameOver, // We dont use this for anything yet :)
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CameraState {
    #[default]
    CarCam,
    FreeCam
}

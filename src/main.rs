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
        // Game Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Game Systems
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera, exit_game))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu, // State for the main menu
    Game,     // State for when the game is running
    GameOver, // We dont use this for anything yet :)
}

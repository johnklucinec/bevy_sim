use bevy::prelude::*;
mod game;
mod main_menu;
mod systems;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;
use game::car::car::*;
use game::car::physics::*;

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
        .add_systems(Startup, 
            (
                setup,
                spawn_car
            )
        )
        .add_systems(
            Update,
            (
                transition_to_game_state,
                transition_to_main_menu_state,
                move_car,
                move_camera,
                exit_game,
            ),
        )
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu, // State for the main menu
    Game,     // State for when the game is running
    GameOver, // We dont use this for anything yet
}

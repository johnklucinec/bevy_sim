use crate::game::ui::pause_menu::components::{
    MainMenuButton, PauseMenu, QuitButton, ResumeButton,
};
use crate::game::ui::pause_menu::styles::*;
use bevy::prelude::*;

/// Spawns the pause menu when entering the MainMenu state.
pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_pause_menu(&mut commands, &asset_server);
}

/// Despawns the pause menu when exiting the PauseMenu state.
pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            pause_menu_parent_style(),
            //PauseMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    pause_menu_style(),
                    BackgroundColor(BACKGROUND_COLOR),
                    BorderColor(Color::BLACK),
                    BorderRadius::all(Val::Px(10.0)),
                    PauseMenu {},
                ))
                .with_children(|parent| {
                    // ====== Resume Button ======
                    parent
                        .spawn((
                            Button,
                            button_style(),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(NORMAL_BUTTON),
                            ResumeButton {},
                        ))
                        .with_child((
                            Text::new("Resume"),
                            get_text_style(28.0, asset_server),
                            TextColor(TEXT_COLOR),
                        ));
                    // ====== Main Menu Button ======
                    parent
                        .spawn((
                            Button,
                            button_style(),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(NORMAL_BUTTON),
                            MainMenuButton {},
                        ))
                        .with_child((
                            Text::new("Main Menu"),
                            get_text_style(28.0, asset_server),
                            TextColor(TEXT_COLOR),
                        ));
                    // ====== Quit Button ======
                    parent
                        .spawn((
                            Button,
                            button_style(),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(10.0)),
                            BackgroundColor(NORMAL_BUTTON),
                            QuitButton {},
                        ))
                        .with_child((
                            Text::new("Quit"),
                            get_text_style(28.0, asset_server),
                            TextColor(TEXT_COLOR),
                        ));
                });
        })
        .id()
}

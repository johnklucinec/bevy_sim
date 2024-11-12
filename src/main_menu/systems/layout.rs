use crate::main_menu::components::*;
use crate::main_menu::styles::*;
use bevy::prelude::*;

/// Spawns the main menu when entering the MainMenu state.
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

/// Despawns the main menu when exiting the MainMenu state.
pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

/// This function constructs the main menu interface, including a title with images,
/// a play button, and a quit button.
///
/// # Arguments
///
/// * `commands` - A mutable reference to the ECS command buffer for entity creation.
/// * `asset_server` - A reference to the asset server resource for loading textures.
///
/// # Returns
///
/// Returns the `Entity` ID of the root main menu container.
pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((menu_style(), BackgroundColor(BACKGROUND_COLOR), MainMenu {}))
        .with_children(|parent| {
            // ====== Title ======
            parent
                .spawn(((
                    title_style(),
                    BorderRadius::MAX,
                    BackgroundColor(BACKGROUND_COLOR),
                ),))
                .with_children(|parent| {
                    // ====== Image 1 ======
                    parent.spawn((
                        UiImage::new(asset_server.load("sprites/beaver.png")),
                        beaver_image_style(),
                    ));
                    // ====== Text ======
                    parent.spawn((
                        Text::new("Bevy Simulator"),
                        get_text_style(45.0, asset_server),
                        TextColor(TEXT_COLOR),
                    ));
                    // ====== Image 2 ======
                    parent.spawn((
                        beaver_image_style(),
                        UiImage::new(asset_server.load("sprites/beaver.png")),
                    ));
                });

            // ====== Play Button ======
            parent
                .spawn((
                    Button,
                    button_style(),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    PlayButton {},
                ))
                .with_child((
                    Text::new("Play"),
                    get_text_style(33.0, &asset_server),
                    TextColor(TEXT_COLOR),
                ));

            // ====== Quit Button ======
            parent
                .spawn((
                    Button,
                    button_style(),
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    QuitButton {},
                ))
                .with_child((
                    Text::new("Quit"),
                    get_text_style(33.0, &asset_server),
                    TextColor(TEXT_COLOR),
                ));
        })
        .id()
}

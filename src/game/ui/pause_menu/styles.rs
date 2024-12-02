use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);
pub const NORMAL_BUTTON: Color = Color::srgb(0.16, 0.17, 0.18);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.27, 0.29);
pub const PRESSED_BUTTON: Color = Color::srgb(0.21, 0.22, 0.25);

pub const SECONDARY_BUTTON: Color = Color::srgb(0.56, 0.22, 0.0);
pub const SECONDARY_HOVERED_BUTTON: Color = Color::srgb(0.68, 0.39, 0.18);
pub const SECONDARY_PRESSED_BUTTON: Color = Color::srgb(0.44, 0.16, 0.0);

pub const TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

/// Pause menu parent node style
pub fn pause_menu_parent_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..Node::default()
    }
}
/// Menu style and layout
pub fn pause_menu_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Auto,
        height: Val::Auto,
        border: UiRect::all(Val::Px(3.0)),
        column_gap: Val::Px(10.0),
        row_gap: Val::Px(10.0),
        padding: UiRect {
            left: Val::Px(10.0),
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            bottom: Val::Px(10.0),
        },
        ..Node::default()
    }
}

/// Menu button style and layout
pub fn button_style() -> Node {
    Node {
        width: Val::Px(250.0),
        height: Val::Px(50.0),
        border: UiRect::all(Val::Px(3.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Node::default()
    }
}

/// Function to get the text style and layout
///
/// # Arguments
///
/// * `font_size` - A `f32` that specifies the size of the font.
/// * `asset_server` - A reference to the `AssetServer` resource to load the font asset.
///
/// # Returns
///
/// * `TextFont` - A `TextFont` struct with the specified font size and loaded font.
pub fn get_text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextFont {
    TextFont {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size,
        ..TextFont::default()
    }
}

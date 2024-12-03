use bevy::prelude::*;

use crate::game::camera::{VIEWPORT_POSITION, VIEWPORT_SIZE};

pub const BACKGROUND_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);
//pub const TEXT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

/// Pause menu parent node style
pub fn camera_view_parent_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(VIEWPORT_SIZE[0] as f32 + (VIEWPORT_POSITION[0] as f32 * 2.0)),
        height: Val::Px(VIEWPORT_SIZE[1] as f32 + (VIEWPORT_POSITION[0] as f32 * 2.0)),
        ..Node::default()
    }
}
/// Camera View Box Border
pub fn camera_view_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        border: UiRect::all(Val::Px(VIEWPORT_POSITION[0] as f32)),
        ..Node::default()
    }
}

// /// Function to get the text style and layout
// ///
// /// # Arguments
// ///
// /// * `font_size` - A `f32` that specifies the size of the font.
// /// * `asset_server` - A reference to the `AssetServer` resource to load the font asset.
// ///
// /// # Returns
// ///
// /// * `TextFont` - A `TextFont` struct with the specified font size and loaded font.
// pub fn get_text_style(font_size: f32, asset_server: &Res<AssetServer>) -> TextFont {
//     TextFont {
//         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//         font_size,
//         ..TextFont::default()
//     }
// }

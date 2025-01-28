use bevy::prelude::*;

/// Pause menu parent node style
pub fn speedometer_parent_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexEnd,
        align_items: AlignItems::FlexEnd,
        position_type: PositionType::Absolute,
        bottom: Val::Px(0.0),
        right: Val::Px(0.0),
        width: Val::Px(300.0),
        height: Val::Px(300.0),
        ..Node::default()
    }
}

/// Camera View Box Border
pub fn speedometer_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        //border: UiRect::all(Val::Px(VIEWPORT_POSITION[0] as f32)),
        ..Node::default()
    }
}

// Image style and layout
pub fn speedometer_image_style() -> Node {
    Node {
        width: Val::Px(300.0),
        height: Val::Px(300.0),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Node::default()
    }
}

// Image style and layout
pub fn needle_image_style() -> Node {
    Node {
        width: Val::Px(150.0),
        height: Val::Px(150.0),
        left: Val::Px(70.0),
        top: Val::Px(100.0),
        position_type: PositionType::Absolute,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
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

/// Author: John Klucinec (@johnklucinec)
use crate::game::ui::free_cam_text::components::CameraText;
use crate::CameraState;
use crate::game::ui::HUDOverlayState;
use bevy::prelude::*;

// Constants
const TEXT_TOP: Val = Val::Px(10.0);
const TEXT_LEFT: Val = Val::Px(10.0);
const FONT_SIZE: f32 = 32.0;

/// Helper function to create the text UI hierarchy
fn build_camera_text(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn(
            Node {
                position_type: PositionType::Absolute,
                top: TEXT_TOP,
                left: TEXT_LEFT,
                ..default()
            }
        )
        .with_children(|parent| {
            parent.spawn((
                Text::new("Speed: "),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: FONT_SIZE,
                    ..default()
                },
                CameraText,
            ));
        })
        .id()
}

/// System for initial camera text setup
pub fn spawn_camera_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_camera_text(&mut commands, &asset_server);
}

/// System to clean up camera_text resources
pub fn despawn_camera_text(
    mut commands: Commands,
    camera_text_query: Query<Entity, With<CameraText>>,
) {
    if let Ok(camera_text_entity) = camera_text_query.get_single() {
        commands.entity(camera_text_entity).despawn_recursive();
    }
}

/// System that updates camera_text each frame
pub fn update_camera_text(
    camera_state: Res<State<CameraState>>,
    mut text_query: Query<&mut Text, With<CameraText>>,
) {
    if let Ok(mut text) = text_query.get_single_mut() {
        match *camera_state.get() {
            CameraState::CarCam => {
                **text = "Cam Mode: Car".to_string();
            },
            CameraState::FreeCam => {
                **text = "Cam Mode: Free".to_string();
            },
        }
    }
}

/// System to toggle visibility based on HUD state
pub fn toggle_camera_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_text_query: Query<Entity, With<CameraText>>,
    state: Res<State<HUDOverlayState>>,
) {
    match state.get() {
        HUDOverlayState::Visible if camera_text_query.is_empty() => {
            build_camera_text(&mut commands, &asset_server);
        }
        HUDOverlayState::Hidden => {
            if let Ok(entity) = camera_text_query.get_single() {
                commands.entity(entity).despawn_recursive();
            }
        }
        _ => {} // No action needed for other state transitions
    }
}

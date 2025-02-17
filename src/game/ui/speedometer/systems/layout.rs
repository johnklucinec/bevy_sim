use crate::game::car::car::*;
use crate::game::ui::speedometer::components::{Speedometer, SpeedometerText};
use crate::game::ui::HUDOverlayState;
use bevy::prelude::*;

// Constants
const SPEEDOMETER_TOP: Val = Val::Px(10.0);
const SPEEDOMETER_RIGHT: Val = Val::Px(10.0);
const FONT_SIZE: f32 = 32.0;
const MPS_TO_MPH: f32 = 2.2369; // Conversion factor from m/s to mph

/// Helper function to create speedometer UI hierarchy
fn build_speedometer(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: SPEEDOMETER_TOP,
                right: SPEEDOMETER_RIGHT,
                ..default()
            },
            Speedometer {},
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Speed: "),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: FONT_SIZE,
                    ..default()
                },
                SpeedometerText,
            ));
        })
        .id()
}

/// System for initial speedometer setup
pub fn spawn_speedometer(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_speedometer(&mut commands, &asset_server);
}

/// System to clean up speedometer resources
pub fn despawn_speedometer(
    mut commands: Commands,
    speedometer_query: Query<Entity, With<Speedometer>>,
) {
    if let Ok(speedometer_entity) = speedometer_query.get_single() {
        commands.entity(speedometer_entity).despawn_recursive();
    }
}

/// System that updates speedometer text each frame
pub fn update_speedometer(
    car_query: Query<&Car>,
    mut text_query: Query<&mut Text, With<SpeedometerText>>,
) {
    if let Ok(car) = car_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            // Convert m/s to MPH and format with gear indicator
            let speed_mph = car.current_speed.abs() * MPS_TO_MPH;
            let gear = match car.gear_mode {
                GearMode::Forward => "D",
                GearMode::Reverse => "R",
            };
            **text = format!("{speed_mph:.1} MPH ({gear})");
        }
    }
}

/// System to toggle visibility based on HUD state
pub fn toggle_speedometer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    speedometer_query: Query<Entity, With<Speedometer>>,
    state: Res<State<HUDOverlayState>>,
) {
    match state.get() {
        HUDOverlayState::Visible if speedometer_query.is_empty() => {
            build_speedometer(&mut commands, &asset_server);
        }
        HUDOverlayState::Hidden => {
            if let Ok(entity) = speedometer_query.get_single() {
                commands.entity(entity).despawn_recursive();
            }
        }
        _ => {} // No action needed for other state transitions
    }
}

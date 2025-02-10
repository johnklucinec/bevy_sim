use bevy::prelude::*;

use crate::game::car::car::*;

#[derive(Component)]
pub struct SpeedometerText;


pub fn spawn_speedometer(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::new("Speed: "),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 32.0,
                ..default()
            },
            Node {
                // Positions the speedometer in the top-right corner
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                ..default()
            },
        ))
        .with_child((
            // Adds a child text span that will display the actual speed
            TextSpan::default(),
            TextFont {
                font_size: 32.0,
                ..default()
            },
            SpeedometerText,
        ));
}

pub fn update_speedometer(
    car_query: Query<&Car>,                                 
    mut query: Query<&mut TextSpan, With<SpeedometerText>>,
) {
    if let Ok(car) = car_query.get_single() {
        if let Ok(mut span) = query.get_single_mut() {
            // convert m/s to mph (1 m/s = 2.2369 MPH)
            let speed_mph = car.current_speed.abs() * 2.2369;
            // get gear
            let gear = match car.gear_mode {
                GearMode::Forward => "D",
                GearMode::Reverse => "R",
            };
            **span = format!("{speed_mph:.1} MPH ({gear})");
        }
    }
}

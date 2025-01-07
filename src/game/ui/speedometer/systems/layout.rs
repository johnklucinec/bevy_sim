use crate::game::ui::speedometer::components::Speedometer;
use crate::game::ui::speedometer::styles::*;
use crate::game::ui::HUDOverlayState;
use bevy::prelude::*;

pub fn spawn_speedometer(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_speedometer(&mut commands, &asset_server);
}

pub fn despawn_speedometer(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<Speedometer>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn build_speedometer(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            speedometer_parent_style(),
            // Camera Style
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    speedometer_style(),
                    //BackgroundColor(BACKGROUND_COLOR),
                    //BorderColor(Color::BLACK),
                    //BorderRadius::all(Val::Px(10.0)),
                    Speedometer {},
                ))
                .with_children(|parent| {
                    // Top image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/speedo.png")),
                        speedometer_image_style(), // You'll need to create this style function
                    ));
                    // Bottom image
                    parent.spawn((
                        ImageNode::new(asset_server.load("sprites/needle.png")),
                        needle_image_style(), // You'll need to create this style function
                    ));
                });
        })
        .id()
}

pub fn toggle_speedometer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    speedometer_query: Query<Entity, With<Speedometer>>,
    state: Res<State<HUDOverlayState>>,
) {
    match state.get() {
        HUDOverlayState::Visible => {
            if speedometer_query.is_empty() {
                build_speedometer(&mut commands, &asset_server);
            }
        }
        HUDOverlayState::Hidden => {
            if let Ok(entity) = speedometer_query.get_single() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

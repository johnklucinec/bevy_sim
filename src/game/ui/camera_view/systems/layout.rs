use crate::game::ui::camera_view::components::CameraViewUi;
use crate::game::ui::camera_view::styles::*;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

pub fn spawn_camera_view_ui(mut commands: Commands) {
    build_camera_view(&mut commands);
}

pub fn despawn_camera_view_ui(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<CameraViewUi>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

fn build_camera_view(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            camera_view_parent_style(), // Camera Style
            RenderLayers::layer(1),
        ))
        .with_children(|parent| {
            parent.spawn((
                camera_view_style(),
                BackgroundColor(BACKGROUND_COLOR),
                BorderColor(Color::BLACK),
                RenderLayers::layer(1),
                BorderRadius::all(Val::Px(10.0)),
                CameraViewUi {},
            ));
        })
        .id()
}

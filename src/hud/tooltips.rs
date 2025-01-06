use super::Tooltip;
use crate::player::PlayerCamera;
use bevy::prelude::*;

pub fn tooltip(
    mut cursor_events: EventReader<CursorMoved>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
    tooltips: Query<(&Tooltip, &Transform)>,
) {
    for event in cursor_events.read() {
        let (camera, camera_transform) = camera_query.single();
        let pos = camera
            .viewport_to_world_2d(camera_transform, event.position)
            .expect("Inconceivable!")
            .as_ivec2();
        tooltips
            .iter()
            .filter(|(_, transform)| transform.translation.truncate().as_ivec2() == pos)
            .for_each(|(tooltip, _)| {
                info!("Tool tip {} at {pos}", tooltip.0);
                // TODO: display the actual tool tip
            });
    }
}

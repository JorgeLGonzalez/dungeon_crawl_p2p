use crate::{
    components::{FieldOfView, FloorTile, WallTile},
    events::RecalculateFovEvent,
};
use bevy::prelude::*;

pub fn recalculate_fov(
    mut fov_query: Query<&mut FieldOfView, With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    tiles: Query<&Transform, With<FloorTile>>,
    walls: Query<&Transform, With<WallTile>>,
) {
    let total_tiles = tiles.iter().count();

    for event in recalculate_events.read() {
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");

        fov.visible_tiles = tiles
            .iter()
            .map(|t| t.translation.truncate())
            .filter(|&tile_pos| event.pos.distance(tile_pos) < fov.radius as f32)
            // TODO line of sight
            .collect();

        info!(
            "Recalculating FOV for {} at {}. See {} tiles of {total_tiles}",
            event.entity,
            event.pos,
            fov.visible_tiles.len()
        );
    }
}

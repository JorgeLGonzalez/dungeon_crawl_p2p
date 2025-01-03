use super::{
    illuminator::{FloorQuery, Illuminator},
    line_of_sight::{BresenhamLineOfSight, WallQuery},
};
use crate::{
    components::{FieldOfView, FovTileMap, Player},
    events::RecalculateFovEvent,
};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn recalculate_fov(
    mut fov_query: Query<&mut FieldOfView, With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor: FloorQuery,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
    walls: WallQuery,
) {
    for event in recalculate_events.read() {
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");
        let viewer = BresenhamLineOfSight::new(event.pos, fov.radius, &walls);

        let visible_tiles: FovTileMap = floor
            .iter()
            .map(|(t, tile, _)| (t.translation.truncate().as_ivec2(), tile))
            .filter(|(floor_pos, _)| viewer.can_see(floor_pos))
            .collect();

        Illuminator::if_local_player(event.entity, &local_players, &players)
            .with_prior_fov(&fov.visible_tiles)
            .illuminate(&mut floor, &visible_tiles);

        fov.visible_tiles = visible_tiles;
    }
}

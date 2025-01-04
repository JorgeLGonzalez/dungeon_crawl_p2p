use super::{
    fov_queries::FovQuery,
    illuminator::{FloorQuery, Illuminator, PlayerQuery},
    line_of_sight::{BresenhamLineOfSight, WallQuery},
    monster_visibility_toggler::{MonsterQuery, MonsterVisibilityToggler},
};
use crate::{components::FovTileMap, events::RecalculateFovEvent};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn recalculate_fov(
    mut fov_query: FovQuery,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor: FloorQuery,
    mut monsters: MonsterQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    walls: WallQuery,
) {
    for event in recalculate_events.read() {
        let mut fov = fov_query
            .get_mut(event.entity)
            .map(|(fov, _)| fov)
            .expect("Inconceivable!");
        let viewer = BresenhamLineOfSight::new(event.pos, fov.radius, &walls);

        let visible_tiles: FovTileMap = floor
            .iter()
            .map(|(t, tile, ..)| (t.translation.truncate().as_ivec2(), tile))
            .filter(|(floor_pos, _)| viewer.can_see(floor_pos))
            .collect();

        Illuminator::if_local_player(event.entity, &local_players, &players)
            .with_prior_fov(&fov.visible_tiles)
            .illuminate(&mut floor, &visible_tiles);

        fov.visible_tiles = visible_tiles.clone();

        MonsterVisibilityToggler::new(event.entity, event.entity_type).toggle(
            &mut monsters,
            &visible_tiles,
            &fov_query,
            &local_players,
        );
    }
}

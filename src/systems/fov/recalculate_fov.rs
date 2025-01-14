use super::{
    fov_queries::FovQuery,
    line_of_sight::{BresenhamLineOfSight, WallQuery},
    visibility_toggler::{VisibilityQuery, VisibilityToggler},
};
use crate::{
    components::{FovRadius, FovTileMap},
    dungeon::{FloorQuery, Illuminator, PlayerQuery},
    events::RecalculateFovEvent,
    player::LocalPlayer,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

/// Recalculate the field of view for the entity that triggered the event.
/// If the event was triggered by the local player moving, illuminate or darken
/// the floor tiles based on the new FOV, and check the visibility of all other
/// relevant entities. Otherwise, we need only update the visibility of the entity
/// that moved and there's no change in floor tile illumination.
pub fn recalculate_fov(
    mut fov_query: FovQuery,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor: FloorQuery,
    mut entities: VisibilityQuery,
    local_players: Res<LocalPlayers>,
    players: PlayerQuery,
    walls: WallQuery,
) {
    for event in recalculate_events.read() {
        let mut fov = fov_query
            .get_mut(event.entity)
            .map(|(fov, _)| fov)
            .expect("Inconceivable!");

        let revised_fov = calculate_fov(event.pos, fov.radius, &floor, &walls);
        let mover_is_local_player = is_local_player(event.entity, &local_players, &players);

        if mover_is_local_player {
            Illuminator::new(&fov.visible_tiles).illuminate(&mut floor, &revised_fov);
        }

        fov.visible_tiles = revised_fov.clone();

        let local_player_fov = local_player_fov(
            mover_is_local_player,
            &revised_fov,
            &fov_query,
            &local_players,
        );

        VisibilityToggler::new(event.entity, local_player_fov)
            .toggle(mover_is_local_player, &mut entities);
    }
}

fn calculate_fov(
    pos: IVec2,
    radius: FovRadius,
    floor: &FloorQuery,
    walls: &WallQuery,
) -> FovTileMap {
    let viewer = BresenhamLineOfSight::new(pos, radius, &walls);

    floor
        .iter()
        .map(|(t, tile, ..)| (t.translation.truncate().as_ivec2(), tile))
        .filter(|(floor_pos, _)| viewer.can_see(floor_pos))
        .collect()
}

fn is_local_player(entity: Entity, local_players: &LocalPlayers, players: &PlayerQuery) -> bool {
    players
        .get(entity)
        .is_ok_and(|player| LocalPlayer::is_local(player, &local_players))
}

fn local_player_fov(
    mover_is_local_player: bool,
    fov: &FovTileMap,
    fov_query: &FovQuery,
    local_players: &LocalPlayers,
) -> HashSet<IVec2> {
    if mover_is_local_player {
        fov.keys().copied().collect()
    } else {
        fov_query
            .iter()
            .find(|(_, player)| player.is_some_and(|p| LocalPlayer::is_local(p, local_players)))
            .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
            .expect("Inconceivable!")
    }
}

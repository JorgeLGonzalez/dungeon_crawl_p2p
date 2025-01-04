use super::{
    illuminator::{FloorQuery, Illuminator, MonsterQuery},
    line_of_sight::{BresenhamLineOfSight, WallQuery},
};
use crate::{
    components::{FieldOfView, FovTileMap, Player},
    events::{FovRecalculationEntityType, RecalculateFovEvent},
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub fn recalculate_fov(
    mut fov_query: Query<(&mut FieldOfView, Option<&Player>), With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor: FloorQuery,
    mut monsters: MonsterQuery,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
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
            .illuminate(&mut floor, &visible_tiles)
            .toggle_monster_visibility(&visible_tiles, &mut monsters);

        fov.visible_tiles = visible_tiles;

        if event.entity_type == FovRecalculationEntityType::Monster {
            for player_id in local_players.0.iter() {
                let player_fov: HashSet<IVec2> = fov_query
                    .iter()
                    .find(|(_, player)| player.is_some_and(|p| p.id == *player_id))
                    .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
                    .expect("Inconceivable!");

                let (_, transform, mut visibility) =
                    monsters.get_mut(event.entity).expect("Inconceivable!");
                let monster_pos = transform.translation.truncate().as_ivec2();

                let expected_visibility = match player_fov.contains(&monster_pos) {
                    false => Visibility::Hidden,
                    true => Visibility::Visible,
                };

                if *visibility != expected_visibility {
                    info!(
                        "Toggling visibility {visibility:?} for monster at {monster_pos} for player {player_id}",

                    );
                    visibility.toggle_visible_hidden();
                }
            }
        }
    }
}

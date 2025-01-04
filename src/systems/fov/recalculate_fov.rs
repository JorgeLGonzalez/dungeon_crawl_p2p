use super::{
    illuminator::{FloorQuery, Illuminator, MonsterQuery},
    line_of_sight::{BresenhamLineOfSight, WallQuery},
};
use crate::{
    components::{FieldOfView, FovTileMap, Player},
    events::{FovRecalculationEntityType, RecalculateFovEvent, ToggleMonsterVisibilityEvent},
};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn recalculate_fov(
    mut fov_query: Query<&mut FieldOfView, With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor: FloorQuery,
    mut monsters: MonsterQuery,
    mut visibility_event: EventWriter<ToggleMonsterVisibilityEvent>,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
    walls: WallQuery,
) {
    for event in recalculate_events.read() {
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");
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
            visibility_event.send(ToggleMonsterVisibilityEvent::new(event.entity));
        }
    }
}

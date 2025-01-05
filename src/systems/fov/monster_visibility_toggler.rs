use super::fov_queries::FovQuery;
use crate::{
    components::{FloorTile, FovTileMap, Monster},
    events::FovRecalculationEntityType,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type MonsterQuery<'w, 's, 't, 'v> =
    Query<'w, 's, (&'t Transform, &'v mut Visibility), (With<Monster>, Without<FloorTile>)>;

pub struct MonsterVisibilityToggler {
    entity: Entity,
    entity_type: FovRecalculationEntityType,
}

impl MonsterVisibilityToggler {
    pub fn new(entity: Entity, entity_type: FovRecalculationEntityType) -> Self {
        Self {
            entity,
            entity_type,
        }
    }

    pub fn toggle(
        &self,
        monsters: &mut MonsterQuery,
        fov: &FovTileMap,
        fov_query: &FovQuery,
        local_players: &LocalPlayers,
    ) {
        match self.entity_type {
            FovRecalculationEntityType::Monster => {
                self.due_to_monster_move(monsters, fov_query, local_players);
            }
            FovRecalculationEntityType::Player => {
                self.due_to_player_move(monsters, fov.keys().copied().collect())
            }
        }
    }

    fn due_to_monster_move(
        &self,
        monsters: &mut MonsterQuery,
        fov_query: &FovQuery,
        local_players: &LocalPlayers,
    ) {
        for player_id in local_players.0.iter() {
            let player_fov: HashSet<IVec2> = fov_query
                .iter()
                .find(|(_, player)| player.is_some_and(|p| p.id == *player_id))
                .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
                .expect("Inconceivable!");

            let (transform, mut visibility) =
                monsters.get_mut(self.entity).expect("Inconceivable!");

            self.toggle_if_needed(&player_fov, transform, &mut visibility);
        }
    }

    fn due_to_player_move(&self, monsters: &mut MonsterQuery, player_fov: HashSet<IVec2>) {
        monsters.iter_mut().for_each(|(transform, mut visibility)| {
            self.toggle_if_needed(&player_fov, transform, &mut visibility);
        });
    }

    fn toggle_if_needed(
        &self,
        player_fov: &HashSet<IVec2>,
        monster_transform: &Transform,
        visibility: &mut Visibility,
    ) {
        let monster_pos = monster_transform.translation.truncate().as_ivec2();
        let expected_visibility = match player_fov.contains(&monster_pos) {
            false => Visibility::Hidden,
            true => Visibility::Visible,
        };

        if *visibility != expected_visibility {
            visibility.toggle_visible_hidden();
        }
    }
}

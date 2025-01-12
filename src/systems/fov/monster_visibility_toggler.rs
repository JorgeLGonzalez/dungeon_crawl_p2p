use super::fov_queries::FovQuery;
use crate::{
    components::{FovTileMap, Monster},
    dungeon::FloorTile,
    events::{FovRecalculationEntityType, RecalculateFovEvent},
    player::LocalPlayer,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type MonsterQuery<'w, 's, 't, 'v> =
    Query<'w, 's, (&'t Transform, &'v mut Visibility), (With<Monster>, Without<FloorTile>)>;

/// Toggle the visibility of monsters based on the new position of the monster
/// or player.
pub struct MonsterVisibilityToggler {
    entity: Entity,
    entity_type: FovRecalculationEntityType,
}

impl MonsterVisibilityToggler {
    pub fn new(event: &RecalculateFovEvent) -> Self {
        Self {
            entity: event.entity,
            entity_type: event.entity_type,
        }
    }

    /// Toggle the visibility of monsters based on the new position of the monster
    /// or player
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

    /// Handle the case where the visibility of the monster may have changed because
    /// the monster moved. In this case we need to get the local player's FOV to
    /// see whether it includes the monster's new position.
    fn due_to_monster_move(
        &self,
        monsters: &mut MonsterQuery,
        fov_query: &FovQuery,
        local_players: &LocalPlayers,
    ) {
        let player_fov: HashSet<IVec2> = fov_query
            .iter()
            .find(|(_, player)| player.is_some_and(|p| LocalPlayer::is_local(p, local_players)))
            .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
            .expect("Inconceivable!");

        let (transform, mut visibility) = monsters.get_mut(self.entity).expect("Inconceivable!");

        self.toggle_if_needed(&player_fov, transform, &mut visibility);
    }

    /// Handle the case where the visibility of the monsters may have changed because
    /// the player moved. In this case we need to check all the monsters to see
    /// whether they are in the player's FOV.
    fn due_to_player_move(&self, monsters: &mut MonsterQuery, player_fov: HashSet<IVec2>) {
        monsters.iter_mut().for_each(|(transform, mut visibility)| {
            self.toggle_if_needed(&player_fov, transform, &mut visibility);
        });
    }

    /// Now that we have the player's FOV and a specific monster's position and
    /// visibility, we can toggle the monster's visibility based on the player's
    /// FOV.
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

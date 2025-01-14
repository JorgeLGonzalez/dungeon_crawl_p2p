use super::fov_queries::FovQuery;
use crate::{
    components::FovTileMap,
    dungeon::{FloorTile, PlayerQuery},
    events::{FovRecalculationEntityType, RecalculateFovEvent},
    player::LocalPlayer,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type VisibilityQuery<'w, 's, 't, 'v> =
    Query<'w, 's, (&'t Transform, &'v mut Visibility), Without<FloorTile>>;

/// Toggle the visibility of entities based on their position in the player's FOV.
pub struct VisibilityToggler {
    entity: Entity,
    entity_type: FovRecalculationEntityType,
    is_local_player: bool,
    local_player_fov: HashSet<IVec2>,
}

impl VisibilityToggler {
    pub fn new(
        event: &RecalculateFovEvent,
        fov: &FovTileMap,
        fov_query: &FovQuery,
        local_players: &LocalPlayers,
        players: &PlayerQuery,
    ) -> Self {
        let is_local_player = Self::is_local_player(event.entity, local_players, players);
        let local_player_fov = match is_local_player {
            false => Self::local_player_fov(fov_query, local_players),
            true => fov.keys().copied().collect(),
        };

        Self {
            entity: event.entity,
            entity_type: event.entity_type,
            is_local_player,
            local_player_fov,
        }
    }

    /// Toggle the visibility of entities based on their position in the player's
    /// FOV.
    pub fn toggle(&self, entities: &mut VisibilityQuery) {
        if self.is_local_player {
            self.due_to_player_move(entities);
        } else {
            self.due_to_monster_move(entities);
        }
    }

    /// Handle the case where the visibility of the monster may have changed because
    /// the monster moved. In this case we need to get the local player's FOV to
    /// see whether it includes the monster's new position.
    fn due_to_monster_move(&self, entities: &mut VisibilityQuery) {
        let (transform, mut visibility) = entities.get_mut(self.entity).expect("Inconceivable!");

        self.toggle_if_needed(transform, &mut visibility);
    }

    /// Handle the case where the visibility of the entities may have changed because
    /// the player moved. In this case we need to check all entities to see
    /// whether they are in the player's FOV.
    fn due_to_player_move(&self, entities: &mut VisibilityQuery) {
        entities.iter_mut().for_each(|(transform, mut visibility)| {
            self.toggle_if_needed(transform, &mut visibility);
        });
    }

    fn is_local_player(
        entity: Entity,
        local_players: &LocalPlayers,
        players: &PlayerQuery,
    ) -> bool {
        players
            .get(entity)
            .is_ok_and(|player| LocalPlayer::is_local(player, &local_players))
    }

    fn local_player_fov(fov_query: &FovQuery, local_players: &LocalPlayers) -> HashSet<IVec2> {
        fov_query
            .iter()
            .find(|(_, player)| player.is_some_and(|p| LocalPlayer::is_local(p, local_players)))
            .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
            .expect("Inconceivable!")
    }

    /// Now that we have the player's FOV and a specific monster's position and
    /// visibility, we can toggle the monster's visibility based on the player's
    /// FOV.
    fn toggle_if_needed(&self, entity_transform: &Transform, visibility: &mut Visibility) {
        let entity_pos = entity_transform.translation.truncate().as_ivec2();
        let expected_visibility = match self.local_player_fov.contains(&entity_pos) {
            false => Visibility::Hidden,
            true => Visibility::Visible,
        };

        if *visibility != expected_visibility {
            visibility.toggle_visible_hidden();
        }
    }
}

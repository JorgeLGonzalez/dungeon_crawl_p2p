use super::fov_queries::FovQuery;
use crate::{
    components::FovTileMap,
    dungeon::FloorTile,
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
}

impl VisibilityToggler {
    pub fn new(event: &RecalculateFovEvent) -> Self {
        Self {
            entity: event.entity,
            entity_type: event.entity_type,
        }
    }

    /// Toggle the visibility of entities based on their position in the player's
    /// FOV.
    pub fn toggle(
        &self,
        entities: &mut VisibilityQuery,
        fov: &FovTileMap,
        fov_query: &FovQuery,
        local_players: &LocalPlayers,
    ) {
        // TODO issue is the visibility is not updated for the remote player.
        // If local player moves out of FOV, local does not see remote, but
        // remote still sees local.
        match self.entity_type {
            FovRecalculationEntityType::Monster => {
                self.due_to_monster_move(entities, fov_query, local_players);
            }
            FovRecalculationEntityType::Player => {
                self.due_to_player_move(entities, fov.keys().copied().collect())
            }
        }
    }

    /// Handle the case where the visibility of the monster may have changed because
    /// the monster moved. In this case we need to get the local player's FOV to
    /// see whether it includes the monster's new position.
    fn due_to_monster_move(
        &self,
        monsters: &mut VisibilityQuery,
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

    /// Handle the case where the visibility of the entities may have changed because
    /// the player moved. In this case we need to check all entities to see
    /// whether they are in the player's FOV.
    fn due_to_player_move(&self, entities: &mut VisibilityQuery, player_fov: HashSet<IVec2>) {
        entities.iter_mut().for_each(|(transform, mut visibility)| {
            self.toggle_if_needed(&player_fov, transform, &mut visibility);
        });
    }

    /// Now that we have the player's FOV and a specific monster's position and
    /// visibility, we can toggle the monster's visibility based on the player's
    /// FOV.
    fn toggle_if_needed(
        &self,
        player_fov: &HashSet<IVec2>,
        entity_transform: &Transform,
        visibility: &mut Visibility,
    ) {
        let entity_pos = entity_transform.translation.truncate().as_ivec2();
        let expected_visibility = match player_fov.contains(&entity_pos) {
            false => Visibility::Hidden,
            true => Visibility::Visible,
        };

        if *visibility != expected_visibility {
            visibility.toggle_visible_hidden();
        }
    }
}

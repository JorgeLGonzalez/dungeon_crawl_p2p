use super::fov_queries::FovQuery;
use crate::{components::FovTileMap, dungeon::FloorTile, player::LocalPlayer};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type VisibilityQuery<'w, 's, 't, 'v> =
    Query<'w, 's, (&'t Transform, &'v mut Visibility), Without<FloorTile>>;

/// Toggle the visibility of entities based on their position in the player's FOV.
pub struct VisibilityToggler {
    mover: Entity,
    mover_is_local_player: bool,
    local_player_fov: HashSet<IVec2>,
}

impl VisibilityToggler {
    pub fn new(
        mover: Entity,
        mover_is_local_player: bool,
        fov: &FovTileMap,
        fov_query: &FovQuery,
        local_players: &LocalPlayers,
    ) -> Self {
        let local_player_fov = match mover_is_local_player {
            false => Self::local_player_fov(fov_query, local_players),
            true => fov.keys().copied().collect(),
        };

        Self {
            mover,
            mover_is_local_player,
            local_player_fov,
        }
    }

    /// Toggle the visibility of entities based on their position in the player's
    /// FOV.
    pub fn toggle(&self, entities: &mut VisibilityQuery) {
        if self.mover_is_local_player {
            entities.iter_mut().for_each(|(transform, mut visibility)| {
                self.toggle_if_needed(transform, &mut visibility);
            });
        } else {
            let (transform, mut visibility) = entities.get_mut(self.mover).expect("Inconceivable!");
            self.toggle_if_needed(transform, &mut visibility);
        }
    }

    fn local_player_fov(fov_query: &FovQuery, local_players: &LocalPlayers) -> HashSet<IVec2> {
        fov_query
            .iter()
            .find(|(_, player)| player.is_some_and(|p| LocalPlayer::is_local(p, local_players)))
            .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
            .expect("Inconceivable!")
    }

    /// Change the entity's (e.g. monster, remote player) visibility based on
    /// the local player's FOV
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

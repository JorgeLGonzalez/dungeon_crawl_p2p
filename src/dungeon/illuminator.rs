use super::FloorTile;
use crate::{
    components::{FovTileMap, Player},
    player::LocalPlayer,
    resources::config,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type FloorQuery<'w, 's, 't, 'r, 'v> =
    Query<'w, 's, (&'t Transform, Entity, &'r mut Sprite, &'v mut Visibility), With<FloorTile>>;
pub type PlayerQuery<'w, 's, 'p> = Query<'w, 's, &'p Player>;

/// Illuminate or darken floor tiles based on the local player's FOV.
pub struct Illuminator {
    is_local_player: bool,
    prior_set: HashSet<Entity>,
}

impl Illuminator {
    /// This is only relevant for the local player. The remaining methods ignore
    /// the case where the entity is not the local player.
    pub fn if_local_player(
        entity: Entity,
        local_players: &LocalPlayers,
        players: &PlayerQuery,
    ) -> Self {
        let is_local_player = players
            .get(entity)
            .is_ok_and(|player| LocalPlayer::is_local(player, &local_players));

        Self {
            is_local_player,
            prior_set: HashSet::default(),
        }
    }

    /// Gather the prior FOV tiles into a HashSet
    pub fn with_prior_fov(self, fov: &FovTileMap) -> Self {
        if !self.is_local_player {
            return self;
        }

        Self {
            is_local_player: true,
            prior_set: fov.values().map(|e| *e).collect(),
            ..self
        }
    }

    /// Illuminate the floor tiles that are in the local player's FOV
    /// and darken those no longer in FOV (but leave them visible).
    pub fn illuminate(mut self, floor: &mut FloorQuery, fov: &FovTileMap) {
        if !self.is_local_player {
            return;
        }

        fov.values().for_each(|tile| {
            if self.prior_set.contains(tile) {
                // already illuminated, so remove it from the prior set
                self.prior_set.remove(tile);
            } else {
                let (.., mut sprite, mut visibility) =
                    floor.get_mut(*tile).expect("Inconceivable!");
                sprite.color = config::FLOOR_ILLUMINATED_COLOR;
                *visibility = Visibility::Visible;
            }
        });

        self.darken_discarded_prior(floor);
    }

    /// darken tiles that were previously illuminated and no longer in FOV
    /// At this point the prior FOV set should only contain tiles that should
    /// be darkened.
    fn darken_discarded_prior(&self, floor: &mut FloorQuery) {
        self.prior_set.iter().for_each(|tile| {
            let (.., mut sprite, _) = floor.get_mut(*tile).expect("Inconceivable!");
            sprite.color = config::FLOOR_COLOR;
        });
    }
}

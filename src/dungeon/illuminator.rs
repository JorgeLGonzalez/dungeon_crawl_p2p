use super::FloorTile;
use crate::{config, fov::FovTileMap, player::Player};
use bevy::{prelude::*, utils::hashbrown::HashSet};

pub type FloorQuery<'w, 's, 't, 'r, 'v> =
    Query<'w, 's, (&'t Transform, Entity, &'r mut Sprite, &'v mut Visibility), With<FloorTile>>;
pub type PlayerQuery<'w, 's, 'p> = Query<'w, 's, &'p Player>;

/// Illuminate or darken floor tiles based on the local player's FOV.
pub struct Illuminator {
    prior_set: HashSet<Entity>,
}

impl Illuminator {
    /// This is only relevant for the local player. The remaining methods ignore
    /// the case where the entity is not the local player.
    pub fn new(prior_fov: &FovTileMap) -> Self {
        Self {
            prior_set: prior_fov.values().map(|e| *e).collect(),
        }
    }

    /// Illuminate the floor tiles that are in the local player's FOV
    /// and darken those no longer in FOV (but leave them visible).
    pub fn illuminate(mut self, floor: &mut FloorQuery, fov: &FovTileMap) {
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

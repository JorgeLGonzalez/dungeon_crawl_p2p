use crate::{
    components::{FloorTile, FovTileMap, Player},
    resources::config,
    systems::player::LocalPlayer,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type FloorQuery<'w, 's, 't, 'r, 'v> =
    Query<'w, 's, (&'t Transform, Entity, &'r mut Sprite, &'v mut Visibility), With<FloorTile>>;
pub type PlayerQuery<'w, 's, 'p> = Query<'w, 's, &'p Player>;

pub struct Illuminator {
    is_local_player: bool,
    prior_set: HashSet<Entity>,
}

impl Illuminator {
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

    pub fn illuminate(mut self, floor: &mut FloorQuery, fov: &FovTileMap) {
        if !self.is_local_player {
            return;
        }

        fov.values().for_each(|tile| {
            if self.prior_set.contains(tile) {
                // already illuminated
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
    fn darken_discarded_prior(&self, floor: &mut FloorQuery) {
        self.prior_set.iter().for_each(|tile| {
            let (.., mut sprite, _) = floor.get_mut(*tile).expect("Inconceivable!");
            sprite.color = config::FLOOR_COLOR;
        });
    }
}

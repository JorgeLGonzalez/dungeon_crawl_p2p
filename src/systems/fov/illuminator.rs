use crate::{
    components::{FloorTile, Player},
    resources::config,
    systems::player::LocalPlayer,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub type FloorQuery<'w, 's, 't, 'r> =
    Query<'w, 's, (&'t Transform, Entity, &'r mut Sprite), With<FloorTile>>;

pub struct Illuminator {
    is_local_player: bool,
    prior_set: HashSet<Entity>,
}

impl Illuminator {
    pub fn if_local_player(
        entity: Entity,
        local_players: &LocalPlayers,
        players: &Query<&Player>,
    ) -> Self {
        let is_local_player = players
            .get(entity)
            .is_ok_and(|player| LocalPlayer::is_local(player, &local_players));

        Self {
            is_local_player,
            prior_set: HashSet::default(),
        }
    }

    pub fn with_prior_fov(self, fov: &Vec<Entity>) -> Self {
        if !self.is_local_player {
            return self;
        }

        Self {
            is_local_player: true,
            prior_set: fov.iter().map(|e| *e).collect(),
        }
    }

    pub fn illuminate(&mut self, floor: &mut FloorQuery, fov: &Vec<Entity>) {
        if !self.is_local_player {
            return;
        }

        fov.iter().for_each(|tile| {
            if self.prior_set.contains(tile) {
                self.prior_set.remove(tile);
            } else {
                let (.., mut sprite) = floor.get_mut(*tile).expect("Inconceivable!");
                sprite.color = config::FLOOR_ILLUMINATED_COLOR;
            }
        });

        self.prior_set.iter().for_each(|tile| {
            let (.., mut sprite) = floor.get_mut(*tile).expect("Inconceivable!");
            sprite.color = config::FLOOR_COLOR;
        });
    }
}

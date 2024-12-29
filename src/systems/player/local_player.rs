use crate::{
    components::{Health, HealthUnit, Player},
    resources::config::{self, GameMode},
};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub type PlayersQuery<'w, 's, 'p, 't, 'h> = Query<'w, 's, (&'p Player, &'t Transform, &'h Health)>;

pub struct LocalPlayer {
    pub id: usize,
    pub health: HealthUnit,
    pub pos: Vec2,
}

impl LocalPlayer {
    pub fn new(local_players: &LocalPlayers, players: &PlayersQuery) -> Self {
        let (id, pos, health) = players
            .iter()
            .find(|(p, ..)| {
                config::GAME_MODE == GameMode::SinglePlayer || local_players.0.contains(&p.id)
            })
            .map(|(p, t, h)| (p.id, t.translation.truncate(), h.current))
            .expect("No local player to follow!");

        Self { id, health, pos }
    }
}

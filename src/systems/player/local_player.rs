use crate::{
    components::{Health, HealthUnit, Player},
    game_mode,
    resources::config::GameMode,
};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub type PlayersQuery<'w, 's, 'p, 't, 'h> = Query<'w, 's, (&'p Player, &'t Transform, &'h Health)>;

pub struct LocalPlayer {
    pub health: HealthUnit,
    pub pos: Vec2,
}

impl LocalPlayer {
    pub fn new(local_players: &LocalPlayers, players: &PlayersQuery) -> Self {
        let (pos, health) = players
            .iter()
            .find(|(p, ..)| {
                if game_mode(GameMode::MultiPlayer) {
                    local_players.0.contains(&p.id)
                } else {
                    p.id == 0
                }
            })
            .map(|(_, t, h)| (t.translation.truncate(), h.current))
            .expect("No local player to follow!");

        Self { health, pos }
    }
}

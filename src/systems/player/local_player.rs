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
            .find(|(p, ..)| Self::is_local(p, local_players))
            .map(|(_, t, h)| (t.translation.truncate(), h.current))
            .expect("No local player to follow!");

        Self { health, pos }
    }

    pub fn is_local(player: &Player, local_players: &LocalPlayers) -> bool {
        if game_mode(GameMode::MultiPlayer) {
            local_players.0.contains(&player.id)
        } else {
            player.id == 0
        }
    }
}
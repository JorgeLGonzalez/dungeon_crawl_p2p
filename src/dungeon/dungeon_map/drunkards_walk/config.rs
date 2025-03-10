use super::{NUM_ITEMS, NUM_MONSTERS, PERCENT_FLOOR};
use crate::config;

pub struct DrunkardsWalkConfig {
    pub num_items: usize,
    pub num_monsters: usize,
    pub num_players: usize,
    pub percent_floor: usize,
}

impl Default for DrunkardsWalkConfig {
    fn default() -> Self {
        let num_players = if config::GAME_MODE == config::GameMode::SinglePlayer {
            1
        } else {
            2
        };

        Self {
            num_items: NUM_ITEMS,
            num_monsters: NUM_MONSTERS,
            num_players,
            percent_floor: PERCENT_FLOOR,
        }
    }
}

use super::DungeonPosition;
use crate::resources::config::NUM_MONSTERS;
use bevy::{
    log::trace,
    prelude::{Entity, Resource},
};
use bevy_ggrs::ggrs::Frame;
use std::collections::VecDeque;

/// Used for diagnostics to track monster moves
#[derive(Resource)]
pub struct MonsterMoveTracker {
    pub moves: VecDeque<MonsterMove>,
}

impl MonsterMoveTracker {
    pub fn new() -> Self {
        Self {
            moves: VecDeque::with_capacity(NUM_MONSTERS * 100),
        }
    }

    pub fn push(&mut self, the_move: MonsterMove) {
        if self.moves.len() >= NUM_MONSTERS * 100 {
            self.moves.pop_front();
        }

        trace!(
            "Monster {:?} moved {} to {} at frame {} and rng {}",
            the_move.monster,
            the_move.movement,
            the_move.pos,
            the_move.frame,
            the_move.rng_counter
        );
        self.moves.push_back(the_move);
    }
}

impl Default for MonsterMoveTracker {
    fn default() -> Self {
        MonsterMoveTracker::new()
    }
}

pub struct MonsterMove {
    pub frame: Frame,
    pub monster: Entity,
    pub movement: DungeonPosition,
    pub pos: DungeonPosition,
    pub rng_counter: u128,
}

impl MonsterMove {
    pub fn csv_headings() -> String {
        "monster,frame,movement,pos,rng".to_string()
    }

    pub fn to_csv(&self) -> String {
        let monster = self.monster.index();

        format!(
            "{monster},{},\"{}\",\"{}\",{}",
            self.frame, self.movement, self.pos, self.rng_counter
        )
    }
}

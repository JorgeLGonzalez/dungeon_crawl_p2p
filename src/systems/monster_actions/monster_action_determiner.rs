use crate::{
    components::PlayerId,
    events::{MonsterAttacksEvent, MonsterMovesEvent},
    resources::{config, DungeonPosition, RandomCounter, RandomGenerator},
};
use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

pub enum MonsterAction {
    Attack(MonsterAttacksEvent),
    Move(MonsterMovesEvent),
}

pub type MonsterPositionSet = HashSet<DungeonPosition>;
pub type PlayerPositionMap = HashMap<DungeonPosition, (Entity, PlayerId)>;
pub type WallPositionSet = HashSet<DungeonPosition>;

pub struct MonsterActionDeterminer {
    current_pos: DungeonPosition,
    monster: Entity,
    movement: Vec2,
    rng_counter: RandomCounter,
    target_pos: DungeonPosition,
}

impl MonsterActionDeterminer {
    pub fn new(monster: Entity, current_pos: Vec2) -> Self {
        Self {
            current_pos: DungeonPosition::from_vec2(current_pos),
            monster,
            movement: Vec2::ZERO,
            rng_counter: 0,
            target_pos: DungeonPosition::from_vec2(Vec2::ZERO),
        }
    }

    pub fn attack(&self, players: &PlayerPositionMap) -> Option<MonsterAction> {
        players
            .get(&self.target_pos)
            .map(|(p, id)| self.create_attack_event(*p, *id))
            .map(MonsterAction::Attack)
    }

    pub fn move_monster(
        &self,
        monster_positions: &mut MonsterPositionSet,
        walls: &WallPositionSet,
    ) -> Option<MonsterAction> {
        if monster_positions.contains(&self.target_pos) || walls.contains(&self.target_pos) {
            return None;
        }

        self.update_monster_positions(monster_positions);

        Some(MonsterAction::Move(self.create_move_event()))
    }

    pub fn plan_move(self, rng: &mut RandomGenerator) -> Option<Self> {
        if !rng.gen_bool(config::MONSTER_MOVE_CHANCE) {
            return None;
        }

        let movement = match rng.gen_range(0..4) {
            0 => Vec2::Y,
            1 => Vec2::NEG_Y,
            2 => Vec2::NEG_X,
            3 => Vec2::X,
            _ => unreachable!(),
        };
        let target_pos = DungeonPosition::from_vec2(self.current_pos.to_vec2() + movement);
        let rng_counter = rng.counter;

        Some(Self {
            movement,
            rng_counter,
            target_pos,
            ..self
        })
    }

    fn update_monster_positions(&self, monster_positions: &mut MonsterPositionSet) {
        monster_positions.remove(&self.current_pos);
        monster_positions.insert(self.target_pos);
    }

    fn create_attack_event(&self, player: Entity, player_id: usize) -> MonsterAttacksEvent {
        MonsterAttacksEvent::new(self.monster, player, player_id, self.target_pos.to_vec2())
    }

    fn create_move_event(&self) -> MonsterMovesEvent {
        MonsterMovesEvent::new(
            self.monster,
            self.movement,
            self.target_pos.to_vec2(),
            self.rng_counter,
        )
    }
}

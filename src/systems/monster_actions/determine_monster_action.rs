use crate::{
    events::{MonsterAttacksEvent, MonsterMovesEvent},
    resources::{config, DungeonPosition, RandomGenerator},
};
use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

pub enum MonsterAction {
    Attack(MonsterAttacksEvent),
    Move(MonsterMovesEvent),
}

pub fn determine_monster_action(
    monster: Entity,
    current_pos: Vec2,
    players: &HashMap<DungeonPosition, (Entity, usize)>,
    walls: &HashSet<DungeonPosition>,
    planned: &mut HashSet<DungeonPosition>,
    rng: &mut RandomGenerator,
) -> Option<MonsterAction> {
    if !rng.gen_bool(config::MONSTER_MOVE_CHANCE) {
        return None;
    }

    let helper =
        Helper::new(DungeonPosition::from_vec2(current_pos), monster).determine_target_pos(rng);

    helper
        .attack_player(players)
        .or_else(|| helper.move_monster(walls, planned))
}

struct Helper {
    current_pos: DungeonPosition,
    monster: Entity,
    movement: Vec2,
    rng_counter: u128,
    target_pos: DungeonPosition,
}

impl Helper {
    fn new(current_pos: DungeonPosition, monster: Entity) -> Self {
        Self {
            current_pos,
            monster,
            movement: Vec2::ZERO,
            rng_counter: 0,
            target_pos: DungeonPosition::from_vec2(Vec2::ZERO),
        }
    }

    fn attack_player(
        &self,
        players: &HashMap<DungeonPosition, (Entity, usize)>,
    ) -> Option<MonsterAction> {
        players
            .get(&self.target_pos)
            .map(|(p, id)| self.create_attack_event(*p, *id))
            .map(MonsterAction::Attack)
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

    fn determine_target_pos(self, rng: &mut RandomGenerator) -> Self {
        let movement = match rng.gen_range(0..4) {
            0 => Vec2::Y,
            1 => Vec2::NEG_Y,
            2 => Vec2::NEG_X,
            3 => Vec2::X,
            _ => unreachable!(),
        };
        let target_pos = DungeonPosition::from_vec2(self.current_pos.to_vec2() + movement);

        Self {
            movement,
            rng_counter: rng.counter,
            target_pos,
            ..self
        }
    }

    fn move_monster(
        &self,
        walls: &HashSet<DungeonPosition>,
        planned: &mut HashSet<DungeonPosition>,
    ) -> Option<MonsterAction> {
        if planned.contains(&self.target_pos) || walls.contains(&self.target_pos) {
            return None;
        }

        self.update_plan(planned);

        Some(MonsterAction::Move(self.create_move_event()))
    }

    fn update_plan(&self, planned: &mut HashSet<DungeonPosition>) {
        planned.remove(&self.current_pos);
        planned.insert(self.target_pos);
    }
}

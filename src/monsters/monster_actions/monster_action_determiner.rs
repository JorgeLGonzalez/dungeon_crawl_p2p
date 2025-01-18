use super::*;
use crate::{
    config,
    fov::FieldOfView,
    health::DamageUnit,
    startup::{RandomCounter, RandomGenerator},
};
use bevy::{prelude::*, utils::hashbrown::HashSet};

pub enum MonsterAction {
    Attack(MonsterAttacksEvent),
    Move(MonsterMovesEvent),
}

pub struct MonsterActionDeterminer {
    current_pos: IVec2,
    damage: DamageUnit,
    fov: HashSet<IVec2>,
    is_throttled: bool,
    monster: Entity,
    target_pos: IVec2,
}

impl MonsterActionDeterminer {
    pub fn new(
        damage: DamageUnit,
        fov: &FieldOfView,
        last_action: &LastAction,
        monster: Entity,
        time: &Time,
        transform: &Transform,
    ) -> Self {
        let is_throttled =
            time.elapsed_secs() - last_action.time < config::MONSTER_THROTTLE_SECONDS;

        Self {
            current_pos: transform.translation.truncate().as_ivec2(),
            damage,
            fov: fov.visible_tiles.keys().copied().collect(),
            is_throttled,
            monster,
            target_pos: IVec2::ZERO,
        }
    }

    pub fn create_acted_event(&self) -> MonsterActedEvent {
        MonsterActedEvent::new(self.monster)
    }

    pub fn determine(
        &mut self,
        params: &MonsterActionParams,
        rng: &mut RandomGenerator,
    ) -> Option<MonsterAction> {
        if self.is_throttled {
            unreachable!("Should not have been called. Check is_throttled")
        }

        let MonsterActionParams {
            monsters: monster_positions,
            players,
            walls,
        } = params;

        let valid_moves = self.gather_valid_moves(monster_positions, walls);
        if valid_moves.is_empty() {
            return None;
        }

        self.try_attack(players)
            .map_or_else(
                || self.random_move(rng, &valid_moves),
                |attack_goal| self.chase(attack_goal, &valid_moves),
            )
            .map(|target_pos| {
                self.target_pos = target_pos;

                self.attack(players)
                    .unwrap_or_else(|| self.move_monster(rng.counter))
            })
    }

    pub fn is_throttled(&self) -> bool {
        self.is_throttled
    }

    pub fn old_pos(&self) -> IVec2 {
        self.current_pos
    }

    pub fn sort_key(&self) -> u32 {
        self.monster.index()
    }

    pub fn target_pos(&self) -> IVec2 {
        self.target_pos
    }

    fn attack(&self, players: &PlayerPositionMap) -> Option<MonsterAction> {
        players
            .get(&self.target_pos)
            .map(|(p, id)| self.create_attack_event(*p, *id))
            .map(MonsterAction::Attack)
    }

    fn chase(&self, attack_goal: IVec2, valid_moves: &[IVec2]) -> Option<IVec2> {
        valid_moves
            .iter()
            .min_by(|m0, m1| {
                m0.distance_squared(attack_goal)
                    .cmp(&m1.distance_squared(attack_goal))
            })
            .copied()
    }

    fn create_attack_event(&self, player: Entity, player_id: usize) -> MonsterAttacksEvent {
        MonsterAttacksEvent::new(
            self.monster,
            self.damage,
            player,
            player_id,
            self.target_pos,
        )
    }

    fn gather_valid_moves(
        &self,
        monster_positions: &MonsterPositionSet,
        walls: &WallPositionSet,
    ) -> Vec<IVec2> {
        [IVec2::Y, IVec2::NEG_Y, IVec2::NEG_X, IVec2::X]
            .iter()
            .map(|&step| step + self.current_pos)
            .filter(|t_pos| !walls.contains(t_pos))
            .filter(|t_pos| !monster_positions.contains(t_pos))
            .collect()
    }

    fn move_monster(&self, rng_counter: RandomCounter) -> MonsterAction {
        MonsterAction::Move(MonsterMovesEvent::new(
            self.monster,
            self.target_pos - self.current_pos,
            self.target_pos,
            rng_counter,
        ))
    }

    fn random_move(&self, rng: &mut RandomGenerator, valid_moves: &[IVec2]) -> Option<IVec2> {
        if rng.gen_bool(config::MONSTER_MOVE_CHANCE) {
            valid_moves
                .get(rng.gen_range(0..valid_moves.len()))
                .copied()
        } else {
            None
        }
    }

    fn try_attack(&self, players: &PlayerPositionMap) -> Option<IVec2> {
        players
            .keys()
            .copied()
            .filter(|player_pos| self.fov.contains(player_pos))
            .min_by(|p0, p1| {
                p0.distance_squared(self.current_pos)
                    .cmp(&p1.distance_squared(self.current_pos))
            })
    }
}

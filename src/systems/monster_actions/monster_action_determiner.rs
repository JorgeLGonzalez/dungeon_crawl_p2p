use crate::{
    components::{FieldOfView, LastAction, Monster, Player, PlayerId},
    events::{MonsterAttacksEvent, MonsterMovesEvent},
    resources::{config, RandomCounter, RandomGenerator},
};
use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

pub enum MonsterAction {
    Attack(MonsterAttacksEvent),
    Move(MonsterMovesEvent),
}

pub type MonsterPositionSet = HashSet<IVec2>;
pub type PlayerPositionMap = HashMap<IVec2, (Entity, PlayerId)>;
pub type PlayersQuery<'w, 's, 't, 'p> =
    Query<'w, 's, (&'t Transform, Entity, &'p Player), (With<Player>, Without<Monster>)>;
pub type WallPositionSet = HashSet<IVec2>;

pub struct MonsterActionDeterminer {
    current_pos: IVec2,
    fov: HashSet<IVec2>,
    last_action_time: f32,
    pub monster: Entity,
    movement: IVec2,
    rng_counter: RandomCounter,
    target_pos: IVec2,
}

impl MonsterActionDeterminer {
    pub fn from_query_tuple(
        (transform, fov, last_action, monster): (&Transform, &FieldOfView, &LastAction, Entity),
    ) -> Self {
        Self {
            current_pos: transform.translation.truncate().as_ivec2(),
            fov: fov.visible_tiles.keys().copied().collect(),
            last_action_time: last_action.time,
            monster,
            movement: IVec2::ZERO,
            rng_counter: 0,
            target_pos: IVec2::ZERO,
        }
    }

    pub fn determine(
        &mut self,
        monster_positions: &MonsterPositionSet,
        players: &PlayerPositionMap,
        time: &Time,
        walls: &WallPositionSet,
        rng: &mut RandomGenerator,
    ) -> Option<MonsterAction> {
        if time.elapsed_secs() - self.last_action_time < config::MONSTER_THROTTLE_SECONDS {
            return None;
        }

        let target_pos = players
            .keys()
            .filter(|player_pos| self.fov.contains(*player_pos))
            .min_by(|p0, p1| {
                p0.distance_squared(self.current_pos)
                    .cmp(&p1.distance_squared(self.current_pos))
            })
            .map(|&player_pos| {
                [IVec2::Y, IVec2::NEG_Y, IVec2::NEG_X, IVec2::X]
                    .iter()
                    .map(|&step| step + self.current_pos)
                    .filter(|t_pos| !walls.contains(t_pos))
                    .filter(|t_pos| !monster_positions.contains(t_pos))
                    .min_by(|m0, m1| {
                        m0.distance_squared(player_pos)
                            .cmp(&m1.distance_squared(player_pos))
                    })
                    .unwrap()
            });

        if let Some(target_pos) = target_pos {
            self.movement = target_pos - self.current_pos;
            self.rng_counter = rng.counter;
            self.target_pos = target_pos;
            return self
                .attack(players)
                .or_else(|| self.move_monster(monster_positions, walls));
        }

        if !rng.gen_bool(config::MONSTER_MOVE_CHANCE) {
            return None;
        }

        self.movement = match rng.gen_range(0..4) {
            0 => IVec2::Y,
            1 => IVec2::NEG_Y,
            2 => IVec2::NEG_X,
            3 => IVec2::X,
            _ => unreachable!(),
        };
        self.rng_counter = rng.counter;
        self.target_pos = self.current_pos + self.movement;

        return self
            .attack(players)
            .or_else(|| self.move_monster(monster_positions, walls));
    }

    fn attack(&self, players: &PlayerPositionMap) -> Option<MonsterAction> {
        players
            .get(&self.target_pos)
            .map(|(p, id)| self.create_attack_event(*p, *id))
            .map(MonsterAction::Attack)
    }

    fn move_monster(
        &self,
        monster_positions: &MonsterPositionSet,
        walls: &WallPositionSet,
    ) -> Option<MonsterAction> {
        if monster_positions.contains(&self.target_pos) || walls.contains(&self.target_pos) {
            return None;
        }

        Some(MonsterAction::Move(self.create_move_event()))
    }

    pub fn sort_key(&self) -> u32 {
        self.monster.index()
    }

    pub fn update_monster_positions(&self, monster_positions: &mut MonsterPositionSet) {
        monster_positions.remove(&self.current_pos);
        monster_positions.insert(self.target_pos);
    }

    fn create_attack_event(&self, player: Entity, player_id: usize) -> MonsterAttacksEvent {
        MonsterAttacksEvent::new(self.monster, player, player_id, self.target_pos)
    }

    fn create_move_event(&self) -> MonsterMovesEvent {
        MonsterMovesEvent::new(
            self.monster,
            self.movement,
            self.target_pos,
            self.rng_counter,
        )
    }
}

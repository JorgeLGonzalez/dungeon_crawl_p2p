use crate::{
    components::{FieldOfView, LastAction, Monster, Player, PlayerId},
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
pub type PlayerPositionMap = HashMap<IVec2, (Entity, PlayerId)>;
pub type PlayersQuery<'w, 's, 't, 'p> =
    Query<'w, 's, (&'t Transform, Entity, &'p Player), (With<Player>, Without<Monster>)>;
pub type WallPositionSet = HashSet<DungeonPosition>;

pub struct MonsterActionDeterminer {
    current_pos: DungeonPosition,
    fov: HashSet<IVec2>,
    last_action_time: f32,
    monster: Entity,
    movement: Vec2,
    players: PlayerPositionMap,
    rng_counter: RandomCounter,
    target_pos: DungeonPosition,
}

impl MonsterActionDeterminer {
    pub fn from_query_tuple(
        (transform, fov, last_action, monster): (&Transform, &FieldOfView, &LastAction, Entity),
        players: &PlayersQuery,
    ) -> Self {
        Self {
            current_pos: DungeonPosition::from_vec2(transform.translation.truncate()),
            fov: fov.visible_tiles.keys().copied().collect(),
            last_action_time: last_action.time,
            monster,
            movement: Vec2::ZERO,
            players: create_player_set(players),
            rng_counter: 0,
            target_pos: DungeonPosition::from_vec2(Vec2::ZERO),
        }
    }

    pub fn attack(&self) -> Option<MonsterAction> {
        self.players
            .get(&self.target_pos.to_vec2().as_ivec2())
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

    pub fn plan_move(self, time: &Time, rng: &mut RandomGenerator) -> Option<Self> {
        if time.elapsed_secs() - self.last_action_time < config::MONSTER_THROTTLE_SECONDS {
            return None;
        }

        let monster_pos = self.current_pos.to_vec2().as_ivec2();
        let target_pos = self
            .players
            .keys()
            .filter(|player_pos| self.fov.contains(*player_pos))
            .min_by(|p0, p1| {
                p0.distance_squared(monster_pos)
                    .cmp(&p1.distance_squared(monster_pos))
            })
            .map(|&player_pos| {
                // TODO consider only valid moves
                [IVec2::Y, IVec2::NEG_Y, IVec2::NEG_X, IVec2::X]
                    .iter()
                    .map(|&step| step + monster_pos)
                    .min_by(|m0, m1| {
                        m0.distance_squared(player_pos)
                            .cmp(&m1.distance_squared(player_pos))
                    })
                    .unwrap()
            });

        if let Some(target_pos) = target_pos {
            info!(
                "Monster {} at {} moves to {target_pos} hopefully towards player",
                self.monster, self.current_pos
            );
            return Some(Self {
                movement: (target_pos - monster_pos).as_vec2(),
                target_pos: DungeonPosition::from_vec2(target_pos.as_vec2()),
                ..self
            });
        }

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

    pub fn sort_key(&self) -> u32 {
        self.monster.index()
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

fn create_player_set(players: &PlayersQuery) -> PlayerPositionMap {
    PlayerPositionMap::from_iter(players.iter().map(|(p, player_entity, player)| {
        (
            p.translation.truncate().as_ivec2(),
            (player_entity, player.id),
        )
    }))
}

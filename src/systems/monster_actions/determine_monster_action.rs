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

    let movement = determine_movement(rng);
    let pos_vec2 = current_pos + movement;
    let pos = DungeonPosition::from_vec2(pos_vec2);

    if let Some((player, player_id)) = players.get(&pos) {
        Some(MonsterAction::Attack(MonsterAttacksEvent::new(
            monster, *player, *player_id, pos_vec2,
        )))
    } else if !planned.contains(&pos) && !walls.contains(&pos) {
        update_plan(planned, current_pos, pos);
        Some(MonsterAction::Move(MonsterMovesEvent::new(
            monster,
            movement,
            pos_vec2,
            rng.counter,
        )))
    } else {
        None
    }
}

fn determine_movement(rng: &mut RandomGenerator) -> Vec2 {
    match rng.gen_range(0..4) {
        0 => Vec2::Y,
        1 => Vec2::NEG_Y,
        2 => Vec2::NEG_X,
        3 => Vec2::X,
        _ => unreachable!(),
    }
}

fn update_plan(
    planned: &mut HashSet<DungeonPosition>,
    current_pos: Vec2,
    target_pos: DungeonPosition,
) {
    planned.remove(&DungeonPosition::from_vec2(current_pos));
    planned.insert(target_pos);
}

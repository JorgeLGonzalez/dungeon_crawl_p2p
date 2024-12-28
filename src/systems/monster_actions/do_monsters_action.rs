use crate::{
    components::{Monster, Player, WallTile},
    resources::{config, DungeonPosition, MonsterMove, MonsterMoveTracker, RandomGenerator},
    MonsterAttacksEvent, MonsterMovesEvent,
};
use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};
use bevy_ggrs::RollbackFrameCount;

type MonsterQuery<'w, 's, 't> = Query<'w, 's, (&'t mut Transform, Entity), With<Monster>>;
type PlayersQuery<'w, 's, 't, 'p> =
    Query<'w, 's, (&'t Transform, Entity, &'p Player), (With<Player>, Without<Monster>)>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub fn do_monsters_action(
    mut attack_event: EventWriter<MonsterAttacksEvent>,
    mut monsters: MonsterQuery,
    mut monster_tracker: ResMut<MonsterMoveTracker>,
    mut move_event: EventWriter<MonsterMovesEvent>,
    mut rng: ResMut<RandomGenerator>,
    frame_count: Res<RollbackFrameCount>,
    players: PlayersQuery,
    wall_tiles: WallQuery,
) {
    let walls = create_wall_set(&wall_tiles);
    let players = create_player_set(&players);
    let mut planned = create_current_monster_positions_set(&monsters);
    let frame = frame_count.0;

    // Sort monsters to ensure all p2p clients process moves in the same way
    let mut monsters: Vec<_> = monsters.iter_mut().collect();
    monsters.sort_by_key(|(_, monster_entity)| monster_entity.index());

    for (mut monster, monster_entity, movement, rng_counter) in
        monsters
            .into_iter()
            .filter_map(|(monster, monster_entity)| {
                determine_movement(&mut rng)
                    .map(|(movement, rng_counter)| (monster, monster_entity, movement, rng_counter))
            })
    {
        let pos = DungeonPosition::from_vec2(monster.translation.truncate() + movement);

        if let Some((player, player_id)) = players.get(&pos) {
            attack_event.send(MonsterAttacksEvent::new(
                monster_entity,
                *player,
                *player_id,
                pos.to_vec2(),
            ));
        } else if !planned.contains(&pos) && !walls.contains(&pos) {
            planned.remove(&DungeonPosition::from_vec3(monster.translation));
            planned.insert(pos);
            move_event.send(MonsterMovesEvent::new(monster_entity, pos.to_vec2()));
            monster.translation = pos.to_vec3(config::MONSTER_Z_LAYER);
            monster_tracker.push(MonsterMove {
                frame,
                monster: monster_entity,
                movement: DungeonPosition::from_vec2(movement),
                pos,
                rng_counter,
            });
        }
    }
}

fn create_current_monster_positions_set(monsters: &MonsterQuery) -> HashSet<DungeonPosition> {
    HashSet::<DungeonPosition>::from_iter(
        monsters
            .iter()
            .map(|(m, _)| DungeonPosition::from_vec3(m.translation)),
    )
}

fn create_player_set(players: &PlayersQuery) -> HashMap<DungeonPosition, (Entity, usize)> {
    HashMap::from_iter(players.iter().map(|(p, player_entity, player)| {
        (
            DungeonPosition::from_vec3(p.translation),
            (player_entity, player.id),
        )
    }))
}

fn create_wall_set(walls: &WallQuery) -> HashSet<DungeonPosition> {
    HashSet::<DungeonPosition>::from_iter(
        walls
            .iter()
            .map(|w| DungeonPosition::from_vec3(w.translation)),
    )
}

fn determine_movement(rng: &mut RandomGenerator) -> Option<(Vec2, u128)> {
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

    Some((movement, rng.counter))
}

use super::determine_monster_action::{determine_monster_action, MonsterAction};
use crate::{
    components::{Monster, Player, WallTile},
    events::{MonsterAttacksEvent, MonsterMovesEvent},
    resources::{DungeonPosition, RandomGenerator},
};
use bevy::{
    prelude::*,
    utils::hashbrown::{HashMap, HashSet},
};

type MonsterQuery<'w, 's, 't> = Query<'w, 's, (&'t Transform, Entity), With<Monster>>;
type PlayersQuery<'w, 's, 't, 'p> =
    Query<'w, 's, (&'t Transform, Entity, &'p Player), (With<Player>, Without<Monster>)>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub fn do_monsters_action(
    mut attack_event: EventWriter<MonsterAttacksEvent>,
    mut move_event: EventWriter<MonsterMovesEvent>,
    mut rng: ResMut<RandomGenerator>,
    monsters: MonsterQuery,
    players: PlayersQuery,
    wall_tiles: WallQuery,
) {
    let walls = create_wall_set(&wall_tiles);
    let players = create_player_set(&players);
    let mut planned = create_current_monster_positions_set(&monsters);

    // Sort monsters to ensure all p2p clients process moves in the same way
    let mut monsters: Vec<_> = monsters.iter().collect();
    monsters.sort_by_key(|(_, monster_entity)| monster_entity.index());

    monsters
        .into_iter()
        .filter_map(|(transform, monster)| {
            determine_monster_action(
                monster,
                transform.translation.truncate(),
                &players,
                &walls,
                &mut planned,
                &mut rng,
            )
        })
        .for_each(|action| match action {
            MonsterAction::Attack(e) => {
                attack_event.send(e);
            }
            MonsterAction::Move(e) => {
                move_event.send(e);
            }
        });
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

use super::monster_action_determiner::{
    MonsterAction, MonsterActionDeterminer, MonsterPositionSet, PlayersQuery, WallPositionSet,
};
use crate::{
    components::{FieldOfView, Monster, WallTile},
    events::{MonsterAttacksEvent, MonsterMovesEvent},
    resources::{DungeonPosition, RandomGenerator},
};
use bevy::prelude::*;

type MonsterQuery<'w, 's, 't, 'f> =
    Query<'w, 's, (&'t Transform, &'f FieldOfView, Entity), With<Monster>>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub fn do_monsters_action(
    mut attack_event: EventWriter<MonsterAttacksEvent>,
    mut move_event: EventWriter<MonsterMovesEvent>,
    mut rng: ResMut<RandomGenerator>,
    monsters: MonsterQuery,
    players: PlayersQuery,
    wall_tiles: WallQuery,
) {
    let mut planned = create_current_monster_positions_set(&monsters);
    let walls = create_wall_set(&wall_tiles);

    sorted_determiners(&monsters, &players)
        .into_iter()
        .filter_map(|d| d.plan_move(&mut rng))
        .filter_map(|d| d.attack().or_else(|| d.move_monster(&mut planned, &walls)))
        .for_each(|action| match action {
            MonsterAction::Attack(e) => {
                attack_event.send(e);
            }
            MonsterAction::Move(e) => {
                move_event.send(e);
            }
        });
}

fn create_current_monster_positions_set(monsters: &MonsterQuery) -> MonsterPositionSet {
    MonsterPositionSet::from_iter(
        monsters
            .iter()
            .map(|(m, ..)| DungeonPosition::from_vec3(m.translation)),
    )
}

fn create_wall_set(walls: &WallQuery) -> WallPositionSet {
    WallPositionSet::from_iter(
        walls
            .iter()
            .map(|w| DungeonPosition::from_vec3(w.translation)),
    )
}

/// Create a Vec of [`MonsterActionDeterminer`]s to help process the actions.
/// Sort them monsters to ensure all p2p clients process moves in the same order.
fn sorted_determiners(
    monsters: &MonsterQuery,
    players: &PlayersQuery,
) -> Vec<MonsterActionDeterminer> {
    let mut monsters: Vec<_> = monsters
        .iter()
        .map(|t| MonsterActionDeterminer::from_query_tuple(t, &players))
        .collect();
    monsters.sort_by_key(|d| d.sort_key());

    monsters
}

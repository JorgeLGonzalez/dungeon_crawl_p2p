use super::monster_action_determiner::{
    MonsterAction, MonsterActionDeterminer, MonsterPositionSet, PlayerPositionMap, PlayersQuery,
    WallPositionSet,
};
use crate::{
    components::{FieldOfView, LastAction, Monster, WallTile},
    events::{MonsterActedEvent, MonsterAttacksEvent, MonsterMovesEvent},
    resources::RandomGenerator,
};
use bevy::prelude::*;

type MonsterQuery<'w, 's, 't, 'f, 'a> =
    Query<'w, 's, (&'t Transform, &'f FieldOfView, &'a LastAction, Entity), With<Monster>>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Monster>)>;

pub fn do_monsters_action(
    mut acted_events: EventWriter<MonsterActedEvent>,
    mut attack_event: EventWriter<MonsterAttacksEvent>,
    mut move_event: EventWriter<MonsterMovesEvent>,
    mut rng: ResMut<RandomGenerator>,
    monsters: MonsterQuery,
    players: PlayersQuery,
    time: Res<Time>,
    wall_tiles: WallQuery,
) {
    let mut planned = create_current_monster_positions_set(&monsters);
    let player_set = create_player_set(&players);
    let walls = create_wall_set(&wall_tiles);

    sorted_determiners(&monsters, &time)
        .into_iter()
        .filter(|d| !d.is_throttled())
        .for_each(|mut determiner| {
            let Some(action) = determiner.determine(&planned, &player_set, &walls, &mut rng) else {
                return;
            };

            determiner.update_monster_positions(&mut planned);
            match action {
                MonsterAction::Attack(e) => {
                    attack_event.send(e);
                }
                MonsterAction::Move(e) => {
                    move_event.send(e);
                }
            };

            acted_events.send(MonsterActedEvent::new(determiner.monster));
        });
}

fn create_current_monster_positions_set(monsters: &MonsterQuery) -> MonsterPositionSet {
    MonsterPositionSet::from_iter(
        monsters
            .iter()
            .map(|(m, ..)| m.translation.truncate().as_ivec2()),
    )
}

fn create_player_set(players: &PlayersQuery) -> PlayerPositionMap {
    PlayerPositionMap::from_iter(players.iter().map(|(p, player_entity, player)| {
        (
            p.translation.truncate().as_ivec2(),
            (player_entity, player.id),
        )
    }))
}

fn create_wall_set(walls: &WallQuery) -> WallPositionSet {
    WallPositionSet::from_iter(walls.iter().map(|w| w.translation.truncate().as_ivec2()))
}

/// Create a Vec of [`MonsterActionDeterminer`]s to help process the actions.
/// Sort them monsters to ensure all p2p clients process moves in the same order.
fn sorted_determiners(monsters: &MonsterQuery, time: &Time) -> Vec<MonsterActionDeterminer> {
    let mut monsters: Vec<_> = monsters
        .iter()
        .map(|(transform, fov, last_action, monster)| {
            MonsterActionDeterminer::new(fov, last_action, monster, time, transform)
        })
        .collect();
    monsters.sort_by_key(|d| d.sort_key());

    monsters
}

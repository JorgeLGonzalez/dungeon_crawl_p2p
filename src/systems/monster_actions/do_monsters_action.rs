use super::{
    monster_action_determiner::{MonsterAction, MonsterActionDeterminer},
    monster_action_params::{MonsterActionParams, MonsterQuery, PlayersQuery, WallQuery},
};
use crate::{
    events::{MonsterActedEvent, MonsterAttacksEvent, MonsterMovesEvent},
    resources::RandomGenerator,
};
use bevy::prelude::*;

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
    let mut params = MonsterActionParams::new(&monsters, &players, &wall_tiles);

    sorted_determiners(&monsters, &time)
        .into_iter()
        .filter(|d| !d.is_throttled())
        .for_each(|mut determiner| {
            let Some(action) = determiner.determine(&params, &mut rng) else {
                return;
            };

            match action {
                MonsterAction::Attack(e) => {
                    attack_event.send(e);
                }
                MonsterAction::Move(e) => {
                    move_event.send(e);
                }
            };

            params.update_monster_position(determiner.old_pos(), determiner.target_pos());
            acted_events.send(determiner.create_acted_event());
        });
}

/// Create a Vec of [`MonsterActionDeterminer`]s to help process the actions.
/// Sort them monsters to ensure all p2p clients process moves in the same order.
fn sorted_determiners(monsters: &MonsterQuery, time: &Time) -> Vec<MonsterActionDeterminer> {
    let mut monsters: Vec<_> = monsters
        .iter()
        .map(|(damage, fov, last_action, monster, transform)| {
            MonsterActionDeterminer::new(damage.0, fov, last_action, monster, time, transform)
        })
        .collect();
    monsters.sort_by_key(|d| d.sort_key());

    monsters
}

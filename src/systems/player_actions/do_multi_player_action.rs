use super::maybe_move_player::{maybe_move_player, MonsterQuery, PlayersQuery, WallsQuery};
use crate::{
    resources::config::{self, GgrsSessionConfig},
    PlayerAction,
};
use bevy::{prelude::*, time::Time};
use bevy_ggrs::PlayerInputs;

pub fn do_multi_player_action(
    mut commands: Commands,
    mut players: PlayersQuery,
    monsters: MonsterQuery,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
    time: Res<Time>,
    walls: WallsQuery,
) {
    assert_eq!(
        players.iter().count(),
        config::NUM_PLAYERS,
        "Unexpected player count!"
    );

    for (mut transform, mut movement, player) in &mut players {
        let action = PlayerAction::from(inputs[player.id].0);
        if action != PlayerAction::None {
            maybe_move_player(
                &mut commands,
                action,
                &monsters,
                player.id,
                &time,
                &walls,
                movement.as_mut(),
                transform.as_mut(),
            );
        }
    }
}

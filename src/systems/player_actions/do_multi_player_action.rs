use super::maybe_move_player::{maybe_move_player, MonsterQuery, PlayersQuery, WallsQuery};
use crate::{
    resources::config::{self, GgrsSessionConfig},
    PlayerInputCode,
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
        maybe_move_player(
            &mut commands,
            PlayerInputCode::from_bits(inputs[player.id].0),
            &monsters,
            player.id,
            &time,
            &walls,
            movement.as_mut(),
            transform.as_mut(),
        );
    }
}

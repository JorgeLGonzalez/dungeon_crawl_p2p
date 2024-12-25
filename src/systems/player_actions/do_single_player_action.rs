use super::maybe_move_player::{maybe_move_player, MonsterQuery, PlayersQuery, WallsQuery};
use super::PlayerInputCode;
use bevy::prelude::*;

pub fn do_single_player_action(
    mut commands: Commands,
    mut players: PlayersQuery,
    keys: Res<ButtonInput<KeyCode>>,
    monsters: MonsterQuery,
    time: Res<Time>,
    walls: WallsQuery,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (mut transform, mut movement, player) = players.single_mut();

    maybe_move_player(
        &mut commands,
        PlayerInputCode::from_keys(&keys),
        &monsters,
        player.id,
        &time,
        &walls,
        movement.as_mut(),
        transform.as_mut(),
    );
}

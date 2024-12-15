use crate::{
    components::{Player, PlayerMovement, WallTile},
    resources::{
        config::{self, GgrsSessionConfig},
        InputDirection,
    },
};
use bevy::{
    input::ButtonInput,
    log::info,
    math::Vec2,
    prelude::{KeyCode, Query, Res, Transform, With, Without},
    time::Time,
};
use bevy_ggrs::PlayerInputs;

type PlayersQuery<'w, 's, 't, 'm, 'p> =
    Query<'w, 's, (&'t mut Transform, &'m mut PlayerMovement, &'p Player), With<Player>>;
type WallsQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Player>)>;

pub fn move_players(
    mut players: PlayersQuery,
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
            InputDirection::from_bits(inputs[player.id].0),
            player,
            &time,
            &walls,
            movement.as_mut(),
            transform.as_mut(),
        );
    }
}

pub fn move_single_player(
    mut players: PlayersQuery,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    walls: WallsQuery,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (mut transform, mut movement, player) = players.single_mut();
    maybe_move_player(
        InputDirection::from_keys(&keys),
        player,
        &time,
        &walls,
        movement.as_mut(),
        transform.as_mut(),
    );
}

fn intersects(player: &Vec2, wall: &Transform) -> bool {
    use config::*;

    static PLAYER_SIZE: Vec2 = Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT);
    static WALL_SIZE: Vec2 = Vec2::new(TILE_WIDTH, TILE_HEIGHT);

    let player_min = player - PLAYER_SIZE / 2.0;
    let player_max = player + PLAYER_SIZE / 2.0;
    let wall_min = wall.translation.truncate() - WALL_SIZE / 2.0;
    let wall_max = wall.translation.truncate() + WALL_SIZE / 2.0;

    player_min.x < wall_max.x
        && player_max.x > wall_min.x
        && player_min.y < wall_max.y
        && player_max.y > wall_min.y
}

fn maybe_move_player(
    input: Option<InputDirection>,
    player: &Player,
    time: &Time,
    walls: &WallsQuery,
    movement: &mut PlayerMovement,
    transform: &mut Transform,
) {
    movement.throttle.tick(time.delta());

    if let Some(direction) = input.map(|i| i.to_vec2()) {
        let changed_dir = movement.direction != Some(direction);
        let is_throttled = !changed_dir && !movement.throttle.finished();
        if changed_dir || !is_throttled {
            movement.throttle.reset();
            movement.direction = Some(direction);

            let pos = transform.translation.truncate() + direction;

            let hit_wall = walls.iter().any(|w| intersects(&pos, w));

            if !hit_wall {
                let old_pos = transform.translation.truncate();
                info!("Player {} moves from {:?} to {:?}", player.id, old_pos, pos);
                transform.translation = pos.extend(config::PLAYER_Z_LAYER);
            } else {
                info!("Player {} move to {:?} blocked by a wall", player.id, pos);
            }
        }
    } else {
        movement.direction = None;
    }
}

use crate::{
    components::{MoveDir, Player, WallTile},
    resources::{calculate_direction, config::*, encode_input},
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
    Query<'w, 's, (&'t mut Transform, &'m mut MoveDir, &'p Player), With<Player>>;
type WallsQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Player>)>;

pub fn move_players(
    mut players: PlayersQuery,
    inputs: Res<PlayerInputs<GgrsSessionConfig>>,
    time: Res<Time>,
    walls: WallsQuery,
) {
    assert_eq!(
        players.iter().count(),
        NUM_PLAYERS,
        "Unexpected player count!"
    );

    for (mut transform, mut move_dir, player) in &mut players {
        move_player(
            inputs[player.id].0,
            player,
            &time,
            &walls,
            move_dir.as_mut(),
            transform.as_mut(),
        );
    }
}

pub fn move_single_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    walls: WallsQuery,
    mut players: PlayersQuery,
) {
    assert_eq!(players.iter().count(), 1, "Unexpected player count!");

    let (mut transform, mut move_dir, player) = players.single_mut();
    move_player(
        encode_input(&keys),
        player,
        &time,
        &walls,
        move_dir.as_mut(),
        transform.as_mut(),
    );
}

fn calculate_pos(old_pos: Vec2, direction: Vec2, delta_seconds: f32) -> Vec2 {
    static MIN: Vec2 = Vec2::new(MAP_WIDTH as f32 / 2., MAP_HEIGHT as f32 / 2.);
    static MAX: Vec2 = Vec2::new(
        MAP_WIDTH as f32 / 2. - PLAYER_WIDTH,
        MAP_HEIGHT as f32 / 2. - PLAYER_HEIGHT,
    );

    let move_delta = direction * PLAYER_SPEED * delta_seconds;

    (old_pos + move_delta).clamp(-MIN, MAX)
}

fn intersects(player: &Vec2, wall: &Transform) -> bool {
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

fn move_player(
    input: u8,
    player: &Player,
    time: &Time,
    walls: &WallsQuery,
    move_dir: &mut MoveDir,
    transform: &mut Transform,
) {
    if let Some(direction) = calculate_direction(input) {
        let old_pos = transform.translation.truncate();
        let elapsed_secs = time.delta_secs();
        move_dir.0 = direction;

        let pos = calculate_pos(old_pos, direction, elapsed_secs);
        info!(
            "Player {} moves {:?} from {:?} to {:?}",
            player.id,
            pos - old_pos,
            old_pos,
            pos
        );

        let hit_wall = walls.iter().any(|w| intersects(&pos, w));

        if !hit_wall {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

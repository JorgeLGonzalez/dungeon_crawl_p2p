use crate::{
    components::{Monster, Player, PlayerMovement, WallTile},
    resources::{
        config::{self, GgrsSessionConfig},
        InputDirection,
    },
};
use bevy::{
    input::ButtonInput,
    log::info,
    math::Vec2,
    prelude::{
        Commands, DespawnRecursiveExt, Entity, KeyCode, Query, Res, Transform, With, Without,
    },
    time::Time,
};
use bevy_ggrs::PlayerInputs;

type MonsterQuery<'w, 's, 't> =
    Query<'w, 's, (Entity, &'t Transform), (With<Monster>, Without<Player>)>;
type PlayersQuery<'w, 's, 't, 'm, 'p> =
    Query<'w, 's, (&'t mut Transform, &'m mut PlayerMovement, &'p Player), With<Player>>;
type WallsQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Player>)>;

pub fn move_players(
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
            InputDirection::from_bits(inputs[player.id].0),
            &monsters,
            player.id,
            &time,
            &walls,
            movement.as_mut(),
            transform.as_mut(),
        );
    }
}

pub fn move_single_player(
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
        InputDirection::from_keys(&keys),
        &monsters,
        player.id,
        &time,
        &walls,
        movement.as_mut(),
        transform.as_mut(),
    );
}

enum PlayerAction {
    Attack(Entity),
    Move(Vec2),
}

fn determine_action(
    direction: &Vec2,
    monsters: &MonsterQuery,
    player_id: usize,
    transform: &Transform,
    walls: &WallsQuery,
) -> Option<PlayerAction> {
    let pos = transform.translation.truncate() + direction;
    let hit_wall = walls.iter().any(|w| intersects(&pos, w));
    let attack = monsters
        .iter()
        .find(|(_, m)| m.translation.truncate() == pos)
        .map(|(m, _)| m);

    if hit_wall {
        info!("Player {player_id} move to {pos:?} blocked by a wall");

        None
    } else if let Some(monster) = attack {
        info!("Player {player_id} attacks monster at {pos:?}");

        Some(PlayerAction::Attack(monster))
    } else {
        let old_pos = transform.translation.truncate();
        info!("Player {player_id} moves from {old_pos:?} to {pos:?}");

        Some(PlayerAction::Move(pos))
    }
}

// TODO can simplify this given unit moves, right?
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
    commands: &mut Commands,
    input: Option<InputDirection>,
    monsters: &MonsterQuery,
    player_id: usize,
    time: &Time,
    walls: &WallsQuery,
    movement: &mut PlayerMovement,
    transform: &mut Transform,
) {
    let action = update_movement(input, &time, movement).and_then(|direction| {
        determine_action(&direction, monsters, player_id, &transform, &walls)
    });

    if let Some(action) = action {
        match action {
            PlayerAction::Attack(monster) => {
                info!("Kill monster");
                commands.entity(monster).despawn_recursive();
            }
            PlayerAction::Move(pos) => {
                transform.translation = pos.extend(config::PLAYER_Z_LAYER);
            }
        };
    }
}

/// A player has a PlayerMovement component that tracks the prior movement direction
/// and a throttle timer to allow a short vs long press works well. A short press
/// is throttled so we get a unit move that allows for precision and moving by
/// a single tile so as to align w/ corridors etc. But if the key is pressed
/// long enough, we want more moves, so the throttle is reset once the timer
/// finishes (or if the player changed direction).
/// (If no key is pressed, PlayerMovement.direction is set to None)
///
/// Return the movement direction only if a move is indicated.
fn update_movement(
    input: Option<InputDirection>,
    time: &Time,
    movement: &mut PlayerMovement,
) -> Option<Vec2> {
    movement.throttle.tick(time.delta());

    if let Some(direction) = input.map(|i| i.to_vec2()) {
        let changed_dir = movement.direction != Some(direction);
        let throttled = !changed_dir && !movement.throttle.finished();
        if changed_dir || !throttled {
            movement.throttle.reset();
            movement.direction = Some(direction);

            movement.direction
        } else {
            None
        }
    } else {
        movement.direction = None;

        None
    }
}

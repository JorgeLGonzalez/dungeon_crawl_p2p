use super::PlayerAction;
use crate::{
    components::{Monster, Player, PlayerMovement, WallTile},
    resources::config,
};
use bevy::{log::info, prelude::*};

pub type MonsterQuery<'w, 's, 't> =
    Query<'w, 's, (Entity, &'t Transform), (With<Monster>, Without<Player>)>;
pub type PlayersQuery<'w, 's, 't, 'm, 'p> =
    Query<'w, 's, (&'t mut Transform, &'m mut PlayerMovement, &'p Player), With<Player>>;
pub type WallsQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Player>)>;

pub fn maybe_move_player(
    commands: &mut Commands,
    action: PlayerAction,
    monsters: &MonsterQuery,
    player_id: usize,
    time: &Time,
    walls: &WallsQuery,
    movement: &mut PlayerMovement,
    transform: &mut Transform,
) {
    let action = update_movement(action, time, movement)
        .and_then(|direction| determine_action(&direction, monsters, player_id, transform, walls));

    if let Some(action) = action {
        match action {
            PlayerActionDetail::Attack(monster) => {
                info!("Player {player_id} killed monster {monster:?}");
                commands.entity(monster).despawn_recursive();
            }
            PlayerActionDetail::Move(pos) => {
                transform.translation = pos.extend(config::PLAYER_Z_LAYER);
            }
        };
    }
}

enum PlayerActionDetail {
    Attack(Entity),
    Move(Vec2),
}

fn determine_action(
    direction: &Vec2,
    monsters: &MonsterQuery,
    player_id: usize,
    transform: &Transform,
    walls: &WallsQuery,
) -> Option<PlayerActionDetail> {
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

        Some(PlayerActionDetail::Attack(monster))
    } else {
        let old_pos = transform.translation.truncate();
        info!("Player {player_id} moves from {old_pos:?} to {pos:?}");

        Some(PlayerActionDetail::Move(pos))
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
    action: PlayerAction,
    time: &Time,
    movement: &mut PlayerMovement,
) -> Option<Vec2> {
    movement.throttle.tick(time.delta());

    let direction_maybe = match action {
        PlayerAction::Up => Some(Vec2::Y),
        PlayerAction::Down => Some(Vec2::NEG_Y),
        PlayerAction::Left => Some(Vec2::NEG_X),
        PlayerAction::Right => Some(Vec2::X),
        _ => None,
    };

    if let Some(direction) = direction_maybe {
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

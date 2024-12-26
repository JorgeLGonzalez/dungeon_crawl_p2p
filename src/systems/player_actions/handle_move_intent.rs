use crate::{
    components::{Monster, Player, PlayerMovement, WallTile},
    events::{PlayerAttackEvent, PlayerMoveEvent, PlayerMoveIntentEvent},
    resources::config,
};
use bevy::{log::info, prelude::*};

pub type MonsterQuery<'w, 's, 't> =
    Query<'w, 's, (Entity, &'t Transform), (With<Monster>, Without<Player>)>;
pub type WallsQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, (With<WallTile>, Without<Player>)>;

// A player has a PlayerMovement component that tracks the prior movement direction
// and a throttle timer to allow a short vs long press works well. A short press
// is throttled so we get a unit move that allows for precision and moving by
// a single tile so as to align w/ corridors etc. But if the key is pressed
// long enough, we want more moves, so the throttle is reset once the timer
// finishes (or if the player changed direction).
// (If no key is pressed, PlayerMovement.direction is set to None)
//
// Return the movement direction only if a move is indicated.

/*
- single query for all transforms and optional identifying componets to figure out what?
- add/remove PlayerMovement component? or otherwise react to key release?
*/

pub fn handle_move_intent(
    mut attack_event: EventWriter<PlayerAttackEvent>,
    mut event_reader: EventReader<PlayerMoveIntentEvent>,
    mut move_event: EventWriter<PlayerMoveEvent>,
    mut player_info: Query<(&mut PlayerMovement, &Transform), With<Player>>,
    monsters: MonsterQuery,
    time: Res<Time>,
    walls: WallsQuery,
) {
    event_reader.read().for_each(|event| {
        let (mut prior_movement, transform) = player_info
            .get_mut(event.player)
            .expect("Player not found!");
        prior_movement.throttle.tick(time.delta());
        let changed_direction = prior_movement.direction != Some(event.direction);
        let throttled = !changed_direction && !prior_movement.throttle.finished();
        if changed_direction || !throttled {
            prior_movement.throttle.reset();
            prior_movement.direction = Some(event.direction);

            if let Some(action) = determine_action(event, &monsters, &transform, &walls) {
                match action {
                    PlayerMove::Attack(e) => {
                        attack_event.send(e);
                    }
                    PlayerMove::Move(e) => {
                        move_event.send(e);
                    }
                }
            }
        }
    });
}

enum PlayerMove {
    Attack(PlayerAttackEvent),
    Move(PlayerMoveEvent),
}

fn determine_action(
    event: &PlayerMoveIntentEvent,
    monsters: &MonsterQuery,
    transform: &Transform,
    walls: &WallsQuery,
) -> Option<PlayerMove> {
    let pos = transform.translation.truncate() + event.direction;
    let hit_wall = walls.iter().any(|w| intersects(&pos, w));
    let attack = monsters
        .iter()
        .find(|(_, m)| m.translation.truncate() == pos)
        .map(|(m, _)| m);

    let player_id = event.player_id;
    if hit_wall {
        info!("Player {player_id} move to {pos} blocked by a wall");

        None
    } else if let Some(monster) = attack {
        info!("Player {player_id} attacks monster at {pos}");

        Some(PlayerMove::Attack(PlayerAttackEvent::new(
            player_id, monster,
        )))
    } else {
        let old_pos = transform.translation.truncate();
        info!("Player {player_id} moves from {old_pos} to {pos}");

        Some(PlayerMove::Move(PlayerMoveEvent::new(
            event.player,
            player_id,
            pos,
        )))
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

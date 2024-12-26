use crate::{
    components::{FloorTile, Monster, Player, PlayerMovement, WallTile},
    events::{PlayerAttackEvent, PlayerMoveEvent, PlayerMoveIntentEvent},
    resources::config::{self, PLAYER_Z_LAYER},
};
use bevy::{log::info, prelude::*};

type ObstacleQuery<'w, 's, 't, 'm, 'wt> = Query<
    'w,
    's,
    (
        Entity,
        &'t Transform,
        Option<&'m Monster>,
        Option<&'wt WallTile>,
    ),
    (Without<Player>, Without<FloorTile>),
>;

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
    obstacles: ObstacleQuery,
    time: Res<Time>,
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

            let target_pos = transform.translation.truncate() + event.direction;
            let obstacle = find_obstacle(target_pos, &obstacles);

            if let Some(action) = determine_action(event, target_pos, obstacle) {
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
    target_pos: Vec2,
    obstacle: Option<Obstacle>,
) -> Option<PlayerMove> {
    let PlayerMoveIntentEvent {
        player, player_id, ..
    } = event;
    match obstacle {
        Some(Obstacle::Monster(monster)) => Some(PlayerMove::Attack(PlayerAttackEvent::new(
            *player_id, target_pos, monster,
        ))),
        Some(Obstacle::Wall) => {
            info!("Player {player_id} move to {target_pos} blocked by a wall");

            None
        }
        None => Some(PlayerMove::Move(PlayerMoveEvent::new(
            *player, *player_id, target_pos,
        ))),
    }
}

fn find_obstacle(target_pos: Vec2, obstacles: &ObstacleQuery) -> Option<Obstacle> {
    obstacles
        .iter()
        .filter(|(_, t, ..)| t.translation.truncate() == target_pos)
        .find_map(|(entity, _t, monster, wall)| {
            monster
                .map(|_| Obstacle::Monster(entity))
                .or_else(|| wall.map(|_| Obstacle::Wall))
        })
}

#[derive(PartialEq, Eq, Hash)]
enum Obstacle {
    Monster(Entity),
    Wall,
}

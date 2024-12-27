use crate::{components::*, PlayerAttackEvent, PlayerMoveEvent, PlayerMoveIntentEvent};
use bevy::{math::Vec2, prelude::*};
use std::time::Duration;

pub type ObstacleQuery<'w, 's, 't, 'm, 'wt> = Query<
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

pub enum PlayerMove {
    Attack(PlayerAttackEvent),
    Move(PlayerMoveEvent),
}

/// Helper for `handle_move_intent`.
pub struct MoveIntentHandler {
    event: PlayerMoveIntentEvent,
    ignore: bool,
    target_pos: Vec2,
}

impl MoveIntentHandler {
    pub fn new(event: PlayerMoveIntentEvent, transform: &Transform) -> Self {
        let target_pos = transform.translation.truncate() + event.direction;

        Self {
            event,
            ignore: false,
            target_pos,
        }
    }

    /// Update the player movement and check if the intended move should be throttled.
    /// A player has a PlayerMovement component that tracks the prior movement direction
    /// and a throttle timer to allow a short vs long press to work well. A short press
    /// is throttled so we get a unit move that allows for precision and moving by
    /// a single tile so as to align w/ corridors etc. But if the key is pressed
    /// long enough, we want more moves, so the throttle is reset once the timer
    /// finishes (or if the player changed direction).
    pub fn update_movement(self, prior_movement: &mut PlayerMovement, delta: Duration) -> Self {
        prior_movement.throttle.tick(delta);
        let changed_direction = prior_movement.direction != Some(self.event.direction);
        let throttled = !changed_direction && !prior_movement.throttle.finished();

        let ignore = !changed_direction && throttled;
        if !ignore {
            prior_movement.throttle.reset();
            prior_movement.direction = Some(self.event.direction);
        }

        Self { ignore, ..self }
    }

    /// Determine whether the intended move is an attack on a monster, a simple
    /// move, or is blocked by a wall or is throttled. Invalid/ignored moves return
    /// None.
    pub fn determine_action(&self, obstacles: &ObstacleQuery) -> Option<PlayerMove> {
        if self.ignore {
            return None;
        }

        let PlayerMoveIntentEvent {
            player, player_id, ..
        } = self.event;
        let target_pos = self.target_pos;
        match self.find_obstacle(obstacles) {
            Some(Obstacle::Monster(monster)) => Some(PlayerMove::Attack(PlayerAttackEvent::new(
                player_id, target_pos, monster,
            ))),
            Some(Obstacle::Wall) => {
                info!("Player {player_id} move to {target_pos} blocked by a wall");

                None
            }
            None => Some(PlayerMove::Move(PlayerMoveEvent::new(
                player, player_id, target_pos,
            ))),
        }
    }

    /// Check whether the intended move lands the player in a Wall tile or
    /// a tile occupied by a monster, returning the type of obstacle. Return
    /// None if the tile is obstacle-free.
    fn find_obstacle(&self, obstacles: &ObstacleQuery) -> Option<Obstacle> {
        obstacles
            .iter()
            .filter(|(_, t, ..)| t.translation.truncate() == self.target_pos)
            .find_map(|(entity, _t, monster, wall)| {
                monster
                    .map(|_| Obstacle::Monster(entity))
                    .or_else(|| wall.map(|_| Obstacle::Wall))
            })
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Obstacle {
    Monster(Entity),
    Wall,
}

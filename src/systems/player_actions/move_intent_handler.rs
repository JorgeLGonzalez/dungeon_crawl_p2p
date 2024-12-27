use crate::{components::*, PlayerAttackEvent, PlayerMoveEvent, PlayerMoveIntentEvent};
use bevy::{math::Vec2, prelude::*};

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

pub type PlayerQuery<'w, 's, 't, 'm> =
    Query<'w, 's, (&'t Transform, Option<&'m PlayerMovement>), With<Player>>;

pub enum PlayerMove {
    Attack(PlayerAttackEvent),
    Move(PlayerMoveEvent),
}

/// Helper for `handle_move_intent`.
pub struct MoveIntentHandler {
    event: PlayerMoveIntentEvent,
    target_pos: Vec2,
    pub throttled: bool,
}

impl MoveIntentHandler {
    pub fn new(event: PlayerMoveIntentEvent, players: &PlayerQuery) -> Self {
        let (transform, throttle) = players.get(event.player).expect("Player not found!");
        let target_pos = transform.translation.truncate() + event.direction;

        Self {
            event,
            target_pos,
            throttled: throttle.is_some(),
        }
    }

    /// Determine whether the intended move is an attack on a monster, a simple
    /// move, or is blocked by a wall or is throttled. Invalid/ignored moves return
    /// None.
    pub fn determine_action(&self, obstacles: &ObstacleQuery) -> Option<PlayerMove> {
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

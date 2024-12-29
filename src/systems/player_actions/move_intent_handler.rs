use crate::{
    components::*,
    events::{PlayerAttacksEvent, PlayerMoveIntentEvent, PlayerMovesEvent},
};
use bevy::{math::Vec2, prelude::*};

pub type ObstacleQuery<'w, 's, 't, 'm, 'p, 'wt> = Query<
    'w,
    's,
    (
        &'t Transform,
        Option<&'m Monster>,
        Option<&'p Player>,
        Option<&'wt WallTile>,
        Entity,
    ),
    With<Obstacle>,
>;

pub type PlayerQuery<'w, 's, 't, 'm> =
    Query<'w, 's, (&'t Transform, Option<&'m MoveThrottle>), With<Player>>;

pub enum PlayerMove {
    Attack(PlayerAttacksEvent),
    Move(PlayerMovesEvent),
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
            Some(ObstacleType::Monster(monster)) => Some(PlayerMove::Attack(
                PlayerAttacksEvent::new(player_id, target_pos, monster),
            )),
            Some(ObstacleType::OtherPlayer) => {
                info!("Player {player_id} move to {target_pos} blocked by another player");

                None
            }
            Some(ObstacleType::Wall) => {
                info!("Player {player_id} move to {target_pos} blocked by a wall");

                None
            }
            None => Some(PlayerMove::Move(PlayerMovesEvent::new(
                player, player_id, target_pos,
            ))),
        }
    }

    /// Check whether the intended move lands the player in a Wall tile or
    /// a tile occupied by a monster, returning the type of obstacle. Return
    /// None if the tile is obstacle-free.
    fn find_obstacle(&self, obstacles: &ObstacleQuery) -> Option<ObstacleType> {
        obstacles
            .iter()
            .find(|(t, ..)| t.translation.truncate() == self.target_pos)
            .map(
                |(_, monster, player, wall, entity)| match (monster, player, wall) {
                    (Some(_), ..) => ObstacleType::Monster(entity),
                    (_, Some(_), _) => ObstacleType::OtherPlayer,
                    (.., Some(_)) => ObstacleType::Wall,
                    _ => unreachable!("Unknown obstacle type"),
                },
            )
    }
}

#[derive(PartialEq, Eq, Hash)]
enum ObstacleType {
    Monster(Entity),
    OtherPlayer,
    Wall,
}

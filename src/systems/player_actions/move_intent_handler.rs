use crate::{
    components::*,
    events::{PlayerAttacksEvent, PlayerMoveIntentEvent, PlayerMovesEvent},
};
use bevy::prelude::*;

pub type ObstacleQuery<'w, 's, 't, 'o> = Query<'w, 's, (&'t Transform, &'o Obstacle, Entity)>;
pub type PlayerQuery<'w, 's, 't, 'd, 'm> =
    Query<'w, 's, (&'t Transform, &'d Damage, Option<&'m MoveThrottle>), With<Player>>;

pub enum PlayerMove {
    Attack(PlayerAttacksEvent),
    Move(PlayerMovesEvent),
}

/// Helper for `handle_move_intent`.
pub struct MoveIntentHandler {
    damage: DamageUnit,
    event: PlayerMoveIntentEvent,
    target_pos: IVec2,
    pub throttled: bool,
}

impl MoveIntentHandler {
    pub fn new(event: PlayerMoveIntentEvent, players: &PlayerQuery) -> Self {
        let (transform, damage, throttle) = players.get(event.player).expect("Player not found!");
        let target_pos = transform.translation.truncate().as_ivec2() + event.direction;

        Self {
            damage: damage.0,
            event,
            target_pos,
            throttled: throttle.is_some(),
        }
    }

    /// Determine whether the intended move is an attack on a monster, a simple
    /// move, or is blocked by a wall or another player. Blocked moves return
    /// None.
    pub fn determine_action(&self, obstacles: &ObstacleQuery) -> Option<PlayerMove> {
        let PlayerMoveIntentEvent {
            player, player_id, ..
        } = self.event;
        let target_pos = self.target_pos;

        if let Some((obstacle, entity)) = self.find_obstacle(obstacles) {
            match obstacle {
                Obstacle::Monster => Some(PlayerMove::Attack(PlayerAttacksEvent::new(
                    player_id,
                    target_pos,
                    entity,
                    self.damage,
                ))),
                Obstacle::Player => {
                    trace!("Player {player_id} move to {target_pos} blocked by another player");

                    None
                }
                Obstacle::Wall => {
                    trace!("Player {player_id} move to {target_pos} blocked by a wall");

                    None
                }
            }
        } else {
            Some(PlayerMove::Move(PlayerMovesEvent::new(
                player, player_id, target_pos,
            )))
        }
    }

    /// Check whether an obstacle interferes the intended move.
    fn find_obstacle(&self, obstacles: &ObstacleQuery) -> Option<(Obstacle, Entity)> {
        obstacles
            .iter()
            .find(|(t, ..)| t.translation.truncate().as_ivec2() == self.target_pos)
            .map(|(_, &obstacle, entity)| (obstacle, entity))
    }
}

use crate::components::PlayerMovement;
use bevy::{
    log::debug,
    prelude::{Commands, Entity, Query, Res},
    time::Time,
};

/// Tick movement throttle (i.e. advance timer) and remove if timer has finished.
pub fn tick_move_throttle(
    mut commands: Commands,
    mut throttled_movements: Query<(&mut PlayerMovement, Entity)>,
    time: Res<Time>,
) {
    for (mut movement, entity) in &mut throttled_movements {
        movement.throttle.tick(time.delta());

        if movement.throttle.just_finished() {
            commands.entity(entity).remove::<PlayerMovement>();
            debug!("Removing PlayerMovement throttle");
        }
    }
}

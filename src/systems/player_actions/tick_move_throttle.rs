use crate::components::MoveThrottle;
use bevy::prelude::*;

/// Tick movement throttle (i.e. advance timer) and remove if timer has finished.
pub fn tick_move_throttle(
    mut commands: Commands,
    mut throttled_movements: Query<(&mut MoveThrottle, Entity)>,
    time: Res<Time>,
) {
    for (mut movement, entity) in &mut throttled_movements {
        movement.tick(time.delta());

        if movement.just_finished() {
            commands.entity(entity).remove::<MoveThrottle>();
        }
    }
}

use crate::health::{Healing, Health};
use bevy::prelude::*;

pub fn healing(
    mut commands: Commands,
    mut query: Query<(&mut Healing, &mut Health, Entity)>,
    time: Res<Time>,
) {
    for (mut healing, mut health, entity) in query.iter_mut() {
        healing.tick(time.delta());

        if healing.just_finished() {
            health.current += 1;

            if health.current == health.max {
                commands
                    .get_entity(entity)
                    .expect("Inconceivable!")
                    .remove::<Healing>();
                info!(
                    "Entity {entity} fully healed at {} of {}",
                    health.current, health.max
                );
            } else {
                info!(
                    "Entity {entity} healed to {} of {}",
                    health.current, health.max
                );
            }
        }
    }
}

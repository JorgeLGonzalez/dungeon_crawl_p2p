use super::{events::DrinkPotionEvent, Health};
use crate::prelude::*;

pub fn drink_potion(
    mut commands: Commands,
    mut events: EventReader<DrinkPotionEvent>,
    mut health_query: Query<&mut Health, With<Player>>,
) {
    for DrinkPotionEvent {
        player,
        player_id,
        hp,
    } in events.read()
    {
        let mut health = health_query.get_mut(*player).expect("Player not found");
        let old_health = health.current;
        health.current = (health.current + hp).min(health.max);
        info!(
            "Player {player_id} drank a {hp} HP potion and healed from {old_health} to {} HP",
            health.current
        );

        if health.current >= health.max {
            commands.entity(*player).remove::<Healing>();
            info!("Removing any healing from player {player_id}");
        }
    }
}

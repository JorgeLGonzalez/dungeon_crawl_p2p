use super::GrabItemEvent;
use crate::{items::Grabbable, prelude::*};

pub fn grab_item(
    mut commands: Commands,
    mut grab_events: EventReader<GrabItemEvent>,
    items: Query<(Entity, &Transform), With<Grabbable>>,
    players: Query<&Transform, With<Player>>,
) {
    for event in grab_events.read() {
        let player = players.get(event.player).expect("Player not found");
        let player_pos = player.translation.truncate().as_ivec2();

        let Some(item) = items
            .iter()
            .find(|(_, t)| t.translation.truncate().as_ivec2() == player_pos)
            .map(|(e, ..)| e)
        else {
            continue;
        };

        info!("Player {} grabs item {item:?}", event.player_id);
        commands.entity(item).despawn_recursive();
    }
}

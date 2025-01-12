use super::RevealDungeonCheatEvent;
use crate::components::FloorTile;
use bevy::prelude::*;

pub fn reveal_cheat(
    mut reveal_events: EventReader<RevealDungeonCheatEvent>,
    mut tiles: Query<&mut Visibility, With<FloorTile>>,
) {
    let Some(event) = reveal_events.read().next() else {
        return;
    };

    info!("Reveal cheat requested by player {}", event.requestor_id);

    tiles
        .iter_mut()
        .filter(|v| **v == Visibility::Hidden)
        .for_each(|mut v| {
            *v = Visibility::Visible;
        });

    reveal_events.clear();
}

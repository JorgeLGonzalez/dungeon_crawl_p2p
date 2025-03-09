use super::LocationText;
use crate::{player::PlayerMovesEvent, prelude::*};

pub fn update_location_ui(
    mut events: EventReader<PlayerMovesEvent>,
    mut query: Query<&mut Text, With<LocationText>>,
) {
    let Some(pos) = events.read().last().map(|e| e.pos) else {
        return;
    };

    query.single_mut().0 = format!("({},{})", pos.x, pos.y);
}

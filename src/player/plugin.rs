use super::events::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttacksEvent>()
            .add_event::<PlayerMovesEvent>()
            .add_event::<PlayerMoveIntentEvent>();
    }
}

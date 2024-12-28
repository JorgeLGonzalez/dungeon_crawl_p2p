use crate::{
    components::Monster,
    events::MonsterMovesEvent,
    resources::{config, MonsterMoveTracker},
};
use bevy::prelude::*;
use bevy_ggrs::RollbackFrameCount;

pub fn move_monster(
    mut monster_tracker: ResMut<MonsterMoveTracker>,
    mut move_events: EventReader<MonsterMovesEvent>,
    mut monsters: Query<&mut Transform, With<Monster>>,
    frame_count: Res<RollbackFrameCount>,
) {
    let frame = frame_count.0;

    for event in move_events.read() {
        if let Ok(mut transform) = monsters.get_mut(event.monster) {
            transform.translation = event.pos.extend(config::MONSTER_Z_LAYER);
            monster_tracker.push(frame, event);
        }
    }
}

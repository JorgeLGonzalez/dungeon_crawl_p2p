use crate::{
    components::Monster,
    events::{MonsterMovesEvent, RecalculateFovEvent},
    resources::{config, MonsterMoveTracker},
};
use bevy::prelude::*;
use bevy_ggrs::RollbackFrameCount;

pub fn move_monster(
    mut fov_event: EventWriter<RecalculateFovEvent>,
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
            fov_event.send(RecalculateFovEvent::new(event.monster, event.pos));
        }
    }
}

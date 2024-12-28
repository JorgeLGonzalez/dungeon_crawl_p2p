use crate::{
    components::Monster,
    events::MonsterMovesEvent,
    resources::{config, DungeonPosition, MonsterMove, MonsterMoveTracker},
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
        let MonsterMovesEvent {
            monster,
            movement,
            pos,
            rng_counter,
        } = event;
        if let Ok(mut transform) = monsters.get_mut(*monster) {
            transform.translation = pos.extend(config::MONSTER_Z_LAYER);
            monster_tracker.push(MonsterMove {
                frame,
                monster: *monster,
                movement: DungeonPosition::from_vec2(*movement),
                pos: DungeonPosition::from_vec2(*pos),
                rng_counter: *rng_counter,
            });
        }
    }
}

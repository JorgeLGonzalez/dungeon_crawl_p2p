use super::game_over::game_over;
use crate::{
    dungeon::{DungeonCoreSet, RespawnState, SpawnDungeonSet},
    items::SpawnItemsSet,
    monsters::MonstersCoreSet,
    player::PlayerCoreSet,
    prelude::*,
};
use bevy_ggrs::{GgrsApp, GgrsSchedule, RollbackFrameCount};

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::GameOver), game_over)
            // .add_systems(
            //     GgrsSchedule,
            //     intermission
            //         .run_if(in_state(GameState::Intermission))
            //         .ambiguous_with(PlayerCoreSet)
            //         .ambiguous_with(DungeonCoreSet)
            //         .ambiguous_with(MonstersCoreSet),
            // )
            .add_systems(
                GgrsSchedule,
                exit_dungeon_spawning
                    .run_if(in_state(GameState::DungeonSpawning))
                    .after(SpawnDungeonSet)
                    .after(SpawnItemsSet)
                    // .ambiguous_with(intermission)
                    .ambiguous_with(DungeonCoreSet)
                    .ambiguous_with(MonstersCoreSet)
                    .ambiguous_with(PlayerCoreSet)
                    .ambiguous_with_all(),
            );
        // .add_systems(OnEnter(GameState::Intermission), enter_intermission);

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_resource_with_copy::<GameState>();
            // .checksum_resource_with_hash::<GameState>();
        }
    }
}

fn exit_dungeon_spawning(world: &mut World) {
    let frame = world.resource::<RollbackFrameCount>().0;
    info!("|HIGHLIGHT| Exiting dungeon spawning (frame {frame})");

    world.insert_resource(RespawnState::Complete(frame));
    let mut respawn = world.resource_mut::<RespawnState>();

    let mut next_state: NextState<GameState> = NextState::default();
    next_state.set(GameState::InGame);
    world.insert_resource(next_state);
    world.run_schedule(StateTransition);
    let current_state = world.resource::<State<GameState>>();
    info!("|HIGHLIGHT| State transitioned to {current_state:?} (frame {frame})");
}

fn exit_dungeon_spawning_old(
    mut next_state: ResMut<NextState<GameState>>,
    mut respawn: ResMut<RespawnState>,
    frame: Res<RollbackFrameCount>,
) {
    let frame = frame.0;
    info!("|HIGHLIGHT| Exiting dungeon spawning (frame {frame})");
    next_state.set(GameState::InGame);

    *respawn = RespawnState::Complete(frame);
}

#[derive(Resource)]
struct IntermissionTimer(Timer);

// fn enter_intermission(mut commands: Commands, frame: Res<RollbackFrameCount>) {
//     let frame = frame.0;
//     info!("|HIGHLIGHT| Entering intermission on frame {frame}");
//     commands.insert_resource(IntermissionTimer(Timer::from_seconds(1.0, TimerMode::Once)));
// }

fn intermission(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    timer: Option<ResMut<IntermissionTimer>>,
    frame: Res<RollbackFrameCount>,
    time: Res<Time>,
) {
    let frame = frame.0;
    let Some(mut timer) = timer else {
        info!("|HIGHLIGHT| Entering intermission on frame {frame}");
        commands.insert_resource(IntermissionTimer(Timer::from_seconds(1.0, TimerMode::Once)));
        return;
    };

    if timer.0.tick(time.delta()).just_finished() {
        info!("|HIGHLIGHT| Exiting intermission on frame {frame}");
        next_state.set(GameState::DungeonSpawning);
        commands.remove_resource::<IntermissionTimer>();
    } else {
        let ms = timer.0.elapsed().as_millis();
        if ms % 100 == 0 {
            info!("|HIGHLIGHT| Intermission timer: {ms}ms frame {frame}");
        }
    }
}

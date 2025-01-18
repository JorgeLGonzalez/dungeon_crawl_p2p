use super::{
    camera::{follow_with_camera, setup_camera},
    components::{MoveThrottle, Player},
    player_actions::*,
    spawn_players::spawn_players,
};
use crate::{
    config::{game_mode, GameMode},
    dungeon::{DungeonCoreSet, SpawnDungeonSet},
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule, ReadInputs};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerCoreSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InGame),
            (spawn_players, setup_camera).after(SpawnDungeonSet),
        );

        let core_systems = (
            do_player_action,
            tick_move_throttle,
            stop_moving,
            handle_move_intent,
            attack_monster,
            move_player,
            follow_with_camera,
        )
            .in_set(PlayerCoreSet)
            .chain()
            .ambiguous_with(DungeonCoreSet)
            .ambiguous_with(crate::hud::HudCoreSet)
            .run_if(in_state(GameState::InGame));

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(Update, core_systems);
        } else {
            app.rollback_component_with_clone::<MoveThrottle>()
                .rollback_component_with_copy::<Player>()
                .checksum_component_with_hash::<MoveThrottle>();

            app.add_systems(ReadInputs, read_player_inputs)
                .add_systems(GgrsSchedule, core_systems);
        }
    }
}

use super::camera::{follow_with_camera, setup_camera};
use super::events::*;
use super::player_actions::{
    attack_monster, do_player_action, handle_move_intent, move_player, read_player_inputs,
    stop_moving, tick_move_throttle,
};
use super::spawn_players::spawn_players;
use crate::dungeon::SpawnDungeonSet;
use crate::{dungeon, game_mode, GameMode, GameState};
use bevy::prelude::*;
use bevy_ggrs::{GgrsSchedule, ReadInputs};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerCoreSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttacksEvent>()
            .add_event::<PlayerMovesEvent>()
            .add_event::<PlayerMoveIntentEvent>()
            .add_systems(
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
            .ambiguous_with(dungeon::DungeonCoreSet)
            .ambiguous_with(crate::hud::HudCoreSet)
            .run_if(in_state(GameState::InGame));

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(Update, core_systems);
        } else {
            app.add_systems(ReadInputs, read_player_inputs)
                .add_systems(GgrsSchedule, core_systems);
        }
    }
}

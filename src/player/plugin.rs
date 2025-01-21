use super::{
    camera::{follow_with_camera, setup_camera},
    components::{MoveThrottle, Obstacle, Player},
    events::PlayerEventsPlugin,
    player_actions::*,
    spawn_players::spawn_players,
};
use crate::{
    common,
    config::{game_mode, GameMode},
    dungeon::{DungeonCoreSet, SpawnDungeonSet},
    monsters::MonstersCoreSet,
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, ReadInputs};

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
            grab_item,
        )
            .in_set(PlayerCoreSet)
            .chain()
            .ambiguous_with(DungeonCoreSet)
            .before(MonstersCoreSet);

        common::add_core_systems(app, core_systems);

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_component_with_clone::<MoveThrottle>()
                .checksum_component_with_hash::<MoveThrottle>()
                .rollback_component_with_copy::<Player>()
                .checksum_component_with_hash::<Player>()
                .rollback_component_with_copy::<Obstacle>()
                .checksum_component_with_hash::<Obstacle>();

            app.add_systems(ReadInputs, read_player_inputs);
        }

        app.add_plugins(PlayerEventsPlugin);
    }
}

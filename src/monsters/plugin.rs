use super::{
    components::{LastAction, Monster},
    events::MonstersEventsPlugin,
    monster_actions::*,
    spawn_monsters::spawn_monsters,
};
use crate::{
    common,
    config::{game_mode, GameMode},
    dungeon::DungeonCoreSet,
    fov::FovCoreSet,
    hud::HudCoreSet,
    player::SpawnPlayersSet,
    GameState,
};
use bevy::prelude::*;
use bevy_ggrs::{GgrsApp, GgrsSchedule};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MonstersCoreSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct SpawnMonstersSet;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MonsterMoveTracker>().add_systems(
            OnEnter(GameState::InGame),
            spawn_monsters
                .in_set(SpawnMonstersSet)
                .after(SpawnPlayersSet),
        );

        let core_systems = (
            do_monsters_action,
            attack_player,
            move_monster,
            update_last_action,
        )
            .in_set(MonstersCoreSet)
            .chain()
            .run_if(in_state(GameState::InGame))
            .ambiguous_with(DungeonCoreSet)
            .before(FovCoreSet)
            .before(HudCoreSet);

        common::add_core_systems(app, core_systems);

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_component_with_copy::<LastAction>()
                .rollback_component_with_copy::<Monster>()
                .checksum_component_with_hash::<Monster>();

            app.add_systems(GgrsSchedule, persist_monster_moves.after(move_monster));
        }

        app.add_plugins(MonstersEventsPlugin);
    }
}

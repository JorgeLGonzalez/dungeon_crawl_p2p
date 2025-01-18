use super::{events::*, monster_actions::*, spawn_monsters};
use crate::{dungeon, game_mode, player, GameMode, GameState};
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MonstersCoreSet;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterActedEvent>()
            .add_event::<MonsterAttacksEvent>()
            .add_event::<MonsterMovesEvent>()
            .init_resource::<MonsterMoveTracker>()
            .add_systems(
                OnEnter(GameState::InGame),
                spawn_monsters.after(dungeon::SpawnDungeonSet),
            );

        let core_systems = (
            do_monsters_action,
            attack_player,
            move_monster,
            update_last_action,
        )
            .in_set(MonstersCoreSet)
            .chain()
            .after(player::PlayerCoreSet)
            .before(dungeon::DungeonCoreSet)
            .run_if(in_state(GameState::InGame));

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(Update, core_systems);
        } else {
            app.add_systems(GgrsSchedule, core_systems)
                .add_systems(GgrsSchedule, persist_monster_moves.after(move_monster));
        }
    }
}

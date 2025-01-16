use super::camera::{follow_with_camera, setup_camera};
use super::events::*;
use crate::{dungeon, game_mode, GameMode, GameState};
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerAttacksEvent>()
            .add_event::<PlayerMovesEvent>()
            .add_event::<PlayerMoveIntentEvent>()
            .add_systems(OnEnter(GameState::Startup), setup_camera);

        let core_systems = follow_with_camera
            .ambiguous_with(dungeon::DungeonCoreSet)
            .ambiguous_with(crate::hud::HudCoreSet)
            .run_if(in_state(GameState::InGame));

        if game_mode(GameMode::SinglePlayer) {
            app.add_systems(Update, core_systems);
        } else {
            app.add_systems(GgrsSchedule, core_systems);
        }
    }
}

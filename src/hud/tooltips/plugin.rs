use super::*;
use crate::{
    common, dungeon::ZoomEvent, monsters::MonsterMovesEvent, player::PlayerMovesEvent, prelude::*,
};

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TooltipCoreSet;

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(toggle_tooltip)
            .add_systems(OnEnter(GameState::Startup), spawn_tooltip);

        common::add_core_systems(
            app,
            (
                on_player_move.run_if(on_event::<PlayerMovesEvent>),
                on_monster_move.run_if(on_event::<MonsterMovesEvent>),
                on_mouse_move.run_if(on_event::<CursorMoved>),
                on_zoom.run_if(on_event::<ZoomEvent>),
            )
                .chain()
                .in_set(TooltipCoreSet),
        );
    }
}

use super::*;
use crate::{common, monsters::MonsterMovesEvent, player::PlayerMovesEvent, prelude::*};

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudStartupSet;

#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct HudCoreSet;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(toggle_tooltip).add_systems(
            OnEnter(GameState::Startup),
            (
                setup_camera,
                setup_health_bar,
                spawn_inventory_ui,
                spawn_tooltip,
                spawn_weapon_ui,
            )
                .chain()
                .in_set(HudStartupSet),
        );

        common::add_core_systems(
            app,
            (
                health_bar,
                update_inventory,
                wield_weapon,
                on_local_player_move.run_if(on_event::<PlayerMovesEvent>),
                on_monster_move.run_if(on_event::<MonsterMovesEvent>),
                on_mouse_move.run_if(on_event::<CursorMoved>.nand(on_event::<PlayerMovesEvent>)),
            )
                .chain()
                .in_set(HudCoreSet),
        );
    }
}

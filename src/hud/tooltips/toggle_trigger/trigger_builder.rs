use super::{
    super::queries::*, super::shower::*, mouse_position::MousePosition, tooltip_info::TooltipInfo,
    TooltipToggleTrigger,
};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

/// Builds the proper TooltipToggleTrigger variant based on a mouse movement
/// event.
pub struct TooltipToggleTriggerBuilder {
    /// whether mouse is in local player's FOV
    in_fov: bool,
    mouse_pos: Option<MousePosition>,
    tooltip: Option<TooltipInfo>,
}

impl TooltipToggleTriggerBuilder {
    pub fn new(camera_query: &CameraQuery, windows: &WindowQuery) -> Self {
        Self {
            in_fov: false,
            mouse_pos: MousePosition::try_new(camera_query, windows),
            tooltip: None,
        }
    }

    pub fn build(self, tooltip_entities: &TooltipEntityQuery) -> Option<TooltipToggleTrigger> {
        let tooltip = self.tooltip.expect("Tooltip not set");

        let Some(mouse_pos) = self.mouse_pos else {
            info!("no mouse pos");
            return tooltip.active().then_some(TooltipToggleTrigger::Hide);
        };

        if !self.in_fov {
            // info!("mouse not in FOV");
            return tooltip.active().then_some(TooltipToggleTrigger::Hide);
        }
        // info!("mouse in FOV");

        // mouse moved but not off of active entity
        if tooltip.active() && tooltip.hit_test(mouse_pos.game) {
            info!("mouse still over tooltip entity");
            return None;
        }

        tooltip_entities
            .iter()
            .find(|(.., transform)| hit_test(mouse_pos.game, transform))
            .map(|(entity, label, _)| {
                TooltipDisplayInfo::new(MouseTooltip(mouse_pos.screen), entity, label.0.clone())
            })
            .map(TooltipToggleTrigger::ShowOnMouseCursor)
            .or_else(|| tooltip.active().then_some(TooltipToggleTrigger::Hide))
    }

    pub fn with_player_fov(mut self, local_players: &LocalPlayers, players: &PlayerQuery) -> Self {
        self.in_fov = self
            .mouse_pos
            .as_ref()
            .map_or(false, |mp| mp.in_player_fov(local_players, players));

        self
    }

    pub fn with_tooltip(
        mut self,
        tooltip_ui: &TooltipUIQuery,
        tooltip_entities: &TooltipEntityQuery,
    ) -> Self {
        self.tooltip = Some(TooltipInfo::new(tooltip_ui, tooltip_entities));

        self
    }
}

fn hit_test(mouse_pos: IVec2, target_transform: &Transform) -> bool {
    let tile_pos = target_transform.translation.truncate().as_ivec2();

    mouse_pos == tile_pos
}

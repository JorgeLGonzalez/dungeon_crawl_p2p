use super::*;
use bevy::prelude::*;

pub fn toggle_tooltip(
    trigger: Trigger<TooltipToggleTrigger>,
    mut tooltip_ui: TooltipUIMutQuery,
    hud_camera_query: HudCameraQuery,
) {
    match trigger.event() {
        TooltipToggleTrigger::Hide(hider) => hider.hide(&mut tooltip_ui),
        TooltipToggleTrigger::ShowOnMouseCursor(info) => {
            TooltipShower::new(info).show(&hud_camera_query, &mut tooltip_ui)
        }
        TooltipToggleTrigger::ShowOnPlayer(info) => TooltipShower::new(info).show(&mut tooltip_ui),
    }
}

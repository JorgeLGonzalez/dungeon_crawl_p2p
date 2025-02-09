use super::{
    determiner::TooltipToggleAction,
    queries::{HudCameraQuery, TooltipUIQuery},
    shower::TooltipShower,
};
use bevy::ecs::observer::Trigger;

pub fn toggle_tooltip(
    trigger: Trigger<TooltipToggleAction>,
    mut tooltip_ui: TooltipUIQuery,
    hud_camera_query: HudCameraQuery,
) {
    match trigger.event() {
        TooltipToggleAction::Hide(hider) => hider.hide(&mut tooltip_ui),
        TooltipToggleAction::None => {}
        TooltipToggleAction::ShowOnMouseCursor(info) => {
            TooltipShower::new(info).show(&hud_camera_query, &mut tooltip_ui)
        }
        TooltipToggleAction::ShowOnPlayer(info) => TooltipShower::new(info).show(&mut tooltip_ui),
    }
}

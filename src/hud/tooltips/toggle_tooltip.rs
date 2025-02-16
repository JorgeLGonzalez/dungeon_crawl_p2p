use super::*;
use bevy::prelude::*;

pub fn toggle_tooltip(
    trigger: Trigger<TooltipToggleTrigger>,
    mut tooltip_ui: TooltipUIMutQuery,
    game_camera: GameCameraQuery,
    hud_camera: HudCameraQuery,
) {
    match trigger.event() {
        TooltipToggleTrigger::Hide => hide_tooltip(&mut tooltip_ui),
        TooltipToggleTrigger::Show(info) => {
            TooltipShower::new(info).show(&mut tooltip_ui, &game_camera, &hud_camera)
        }
    }
}

fn hide_tooltip(tooltip_ui: &mut TooltipUIMutQuery) {
    let (mut tooltip_node, mut tooltip_text, mut tooltip) = tooltip_ui.single_mut();
    tooltip_node.display = Display::None;
    tooltip_text.0 = String::new();
    tooltip.entity = None;
}

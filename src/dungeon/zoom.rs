use super::{ZoomDirection, ZoomEvent};
use crate::player::{LocalPlayer, PlayerCamera};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

const ZOOM_STEP: f32 = 0.25;
const ZOOM_MAX: f32 = 10.0;

/// Zoom the local player camera  in or out based on the direction of the event.
/// The event is send by the player when they press the zoom in (shift+Plus) or
/// out (shift+Minus) key.
pub fn zoom(
    mut camera: Query<&mut Projection, With<PlayerCamera>>,
    mut events: EventReader<ZoomEvent>,
    local_players: Res<LocalPlayers>,
) {
    events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.requestor_id, &local_players))
        .map(|e| e.direction)
        .for_each(|direction| {
            let mut camera = camera.single_mut();
            let Projection::Orthographic(ref mut projection) = camera.as_mut() else {
                return;
            };

            projection.scale += determine_zoom_step(projection.scale, direction);
        });
}

fn determine_zoom_step(scale: f32, direction: ZoomDirection) -> f32 {
    match direction {
        ZoomDirection::In if scale > ZOOM_STEP => -ZOOM_STEP,
        ZoomDirection::Out if scale < ZOOM_MAX => ZOOM_STEP,
        ZoomDirection::In => {
            warn!("Cannot zoom in any further");
            0.
        }
        ZoomDirection::Out => {
            warn!("Cannot zoom out any further");
            0.
        }
    }
}

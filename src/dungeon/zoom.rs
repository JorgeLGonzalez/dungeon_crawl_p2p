use super::{ZoomDirection, ZoomEvent};
use crate::player::PlayerCamera;
use bevy::prelude::*;

const ZOOM_STEP: f32 = 0.25;
const ZOOM_MAX: f32 = 10.0;

pub fn zoom(
    mut camera: Query<&mut Projection, With<PlayerCamera>>,
    mut events: EventReader<ZoomEvent>,
) {
    for event in events.read() {
        // TODO ignore for non-local players

        let mut camera = camera.single_mut();
        let Projection::Orthographic(ref mut projection) = camera.as_mut() else {
            continue;
        };

        let scale_step = match event.direction {
            ZoomDirection::In if projection.scale > ZOOM_STEP => Ok(-ZOOM_STEP),
            ZoomDirection::In => Err("Cannot zoom in any further"),
            ZoomDirection::Out if projection.scale < ZOOM_MAX => Ok(ZOOM_STEP),
            ZoomDirection::Out => Err("Cannot zoom out any further"),
        };

        match scale_step {
            Ok(step) => {
                projection.scale += step;
            }
            Err(e) => {
                warn!("{}", e);
            }
        };
    }
}

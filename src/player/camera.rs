mod follow_player;
mod setup_camera;

pub use follow_player::follow_with_camera;
pub use setup_camera::setup_camera;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

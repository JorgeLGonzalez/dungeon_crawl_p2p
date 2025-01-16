mod follow_player;
mod setup_camera;

// TODO make super once we fix all player systems
pub use follow_player::follow_with_camera;
pub(super) use setup_camera::setup_camera;

use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerCamera;

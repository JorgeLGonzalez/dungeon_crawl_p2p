mod camera;
mod components;
mod events;
mod local_player;
mod plugin;

pub use components::{Player, PlayerCamera, PlayerId};
pub use events::{PlayerAttacksEvent, PlayerMoveIntentEvent, PlayerMovesEvent};
pub use local_player::{LocalPlayer, PlayersQuery};

pub use plugin::PlayerPlugin;

// TODO remove
pub use camera::follow_with_camera;

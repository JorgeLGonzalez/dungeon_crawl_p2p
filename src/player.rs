mod camera;
mod events;
mod local_player;
mod plugin;

pub use camera::*;
pub use events::{PlayerAttacksEvent, PlayerMoveIntentEvent, PlayerMovesEvent};
pub use local_player::{LocalPlayer, PlayersQuery};

pub use plugin::PlayerPlugin;

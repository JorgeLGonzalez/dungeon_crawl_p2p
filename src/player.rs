mod camera;
mod components;
mod events;
mod local_player;
mod player_actions;
mod plugin;
mod spawn_players;

pub use components::{MoveThrottle, Obstacle, Player, PlayerCamera, PlayerId};
pub use local_player::{LocalPlayer, PlayersQuery};
pub use plugin::{PlayerCoreSet, PlayerPlugin};

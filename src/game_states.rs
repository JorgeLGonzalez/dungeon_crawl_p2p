mod game_over;
mod plugin;

pub use plugin::GameStatesPlugin;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    GameOver,
    InGame,
    #[default]
    Loading,
    Paused,
    Startup,
}

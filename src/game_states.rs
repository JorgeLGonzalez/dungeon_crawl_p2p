mod game_over;
mod plugin;

pub use plugin::GameStatesPlugin;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    /// Generate the dungeon level and its contents before going into game
    /// after startup or changing levels
    DungeonSpawning,
    GameOver,
    /// Main game loop
    InGame,
    #[default]
    /// Asset loading
    Loading,
    Paused,
    /// Basic startup. Includes waiting for players to join when in multiplayer
    /// mode
    Startup,
}

use super::GameState;
use bevy::prelude::*;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

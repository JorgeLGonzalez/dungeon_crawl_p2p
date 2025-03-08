use super::game_over::game_over;
use crate::prelude::*;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::GameOver), game_over)
            .add_systems(
                Update,
                exit_dungeon_spawning.run_if(in_state(GameState::DungeonSpawning)),
            );
    }
}

fn exit_dungeon_spawning(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::InGame);
}

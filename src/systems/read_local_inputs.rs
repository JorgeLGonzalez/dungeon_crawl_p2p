use crate::resources::config::GgrsSessionConfig;
use bevy::{
    input::ButtonInput,
    prelude::{Commands, KeyCode, Res},
    utils::hashbrown::HashMap,
};
use bevy_ggrs::{LocalInputs, LocalPlayers};

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for player in &local_players.0 {
        let mut input = 0u8;

        if keys.pressed(KeyCode::Space) {
            input += 1 << 0;
        }

        local_inputs.insert(*player, input);
    }

    commands.insert_resource(LocalInputs::<GgrsSessionConfig>(local_inputs));
}

use crate::resources::{config, InputDirection};
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
    let local_inputs = local_players
        .0
        .iter()
        .fold(HashMap::new(), |mut acc, &player_handle| {
            acc.insert(
                player_handle,
                InputDirection::from_keys(&keys)
                    .map(|d| d.to_bits())
                    .unwrap_or_default(),
            );
            acc
        });

    commands.insert_resource(LocalInputs::<config::GgrsSessionConfig>(local_inputs));
}

use crate::resources::{config, PlayerInputCode};
use bevy::{
    input::ButtonInput,
    log::info,
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
            let encoded_input = PlayerInputCode::from_keys(&keys)
                .map(|d| d.to_bits())
                .unwrap_or_default();
            acc.insert(player_handle, encoded_input);

            if PlayerInputCode::from_bits(encoded_input)
                .is_some_and(|i| matches!(i, PlayerInputCode::Snapshot))
            {
                info!("Player {} requested snapshot", player_handle);
            }

            acc
        });

    commands.insert_resource(LocalInputs::<config::GgrsSessionConfig>(local_inputs));
}

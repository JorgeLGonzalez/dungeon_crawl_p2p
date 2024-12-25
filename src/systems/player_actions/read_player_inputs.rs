use super::player_inputs::PlayerInputCode;
use crate::resources::config;
use bevy::{
    input::ButtonInput,
    log::info,
    prelude::{Commands, KeyCode, Res},
    utils::hashbrown::HashMap,
};
use bevy_ggrs::{LocalInputs, LocalPlayers};

/// Used in GGRS mode to determine PlayerAction and store into LocalInputs resource.
/// This will be shared with local and remote players that can then handle the action.
/// LocalPlayers has a single item in P2P mode (and 2 in SyncTest mode).
pub fn read_player_inputs(
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

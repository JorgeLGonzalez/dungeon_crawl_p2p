use super::player_action::PlayerAction;
use crate::resources::config;
use bevy::{prelude::*, utils::hashbrown::HashMap};
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
            let action = PlayerAction::from(keys.as_ref());
            acc.insert(player_handle, action.into());

            acc
        });

    commands.insert_resource(LocalInputs::<config::GgrsSessionConfig>(local_inputs));
}

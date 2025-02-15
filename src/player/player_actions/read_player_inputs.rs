use super::{player_action::PickedItemQuery, PlayerAction};
use crate::config;
use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};

/// Used in GGRS mode to determine PlayerAction and store into LocalInputs resource.
/// This will be shared with local and remote players that can then handle the action.
/// LocalPlayers has a single item in P2P mode (and 2 in SyncTest mode).
pub fn read_player_inputs(
    mut commands: Commands,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
    picked_items: PickedItemQuery,
) {
    let local_inputs = local_players
        .0
        .iter()
        .fold(HashMap::new(), |mut acc, &player_handle| {
            let action = PlayerAction::new(keys.as_mut(), &picked_items);
            acc.insert(player_handle, action.into());

            acc
        });

    commands.insert_resource(LocalInputs::<config::GgrsSessionConfig>(local_inputs));
}

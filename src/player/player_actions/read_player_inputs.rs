use super::{player_action::PickedItemQuery, PlayerAction};
use crate::{config, prelude::GameState};
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
    state: Res<State<GameState>>,
) {
    let local_inputs = local_players
        .0
        .iter()
        .fold(HashMap::new(), |mut acc, &player_handle| {
            let action = (*state == GameState::InGame)
                .then(|| PlayerAction::new(keys.as_mut(), &picked_items))
                .unwrap_or(PlayerAction::None);
            if action != PlayerAction::None {
                info!("Local player {player_handle} action: {action:?}");
            }
            acc.insert(player_handle, action.into());

            acc
        });

    commands.insert_resource(LocalInputs::<config::GgrsSessionConfig>(local_inputs));
}

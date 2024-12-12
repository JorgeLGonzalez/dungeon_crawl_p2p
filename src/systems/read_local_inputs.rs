use crate::resources::create_local_inputs;
use bevy::{
    input::ButtonInput,
    prelude::{Commands, KeyCode, Res},
};
use bevy_ggrs::LocalPlayers;

pub fn read_local_inputs(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    local_players: Res<LocalPlayers>,
) {
    commands.insert_resource(create_local_inputs(&keys, &local_players));
}

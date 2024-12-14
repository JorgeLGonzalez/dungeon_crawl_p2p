use super::config::GgrsSessionConfig;
use bevy::{input::ButtonInput, math::Vec2, prelude::KeyCode, utils::hashbrown::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_FIRE: u8 = 1 << 4;

pub fn calculate_direction(input: u8) -> Option<Vec2> {
    let mut direction = Vec2::ZERO;

    if input & INPUT_UP != 0 {
        direction.y += 1.;
    } else if input & INPUT_DOWN != 0 {
        direction.y -= 1.;
    } else if input & INPUT_LEFT != 0 {
        direction.x -= 1.;
    } else if input & INPUT_RIGHT != 0 {
        direction.x += 1.;
    }

    if direction == Vec2::ZERO {
        None
    } else {
        Some(direction.normalize_or_zero())
    }
}

pub fn create_local_inputs(
    keys: &ButtonInput<KeyCode>,
    players: &LocalPlayers,
) -> LocalInputs<GgrsSessionConfig> {
    let local_inputs = players
        .0
        .iter()
        .fold(HashMap::new(), |mut acc, &player_handle| {
            acc.insert(player_handle, encode_input(keys));
            acc
        });

    LocalInputs::<GgrsSessionConfig>(local_inputs)
}

pub fn encode_input(keys: &ButtonInput<KeyCode>) -> u8 {
    use KeyCode::*;

    let mut input = 0u8;

    if keys.any_just_pressed([ArrowUp, KeyW]) {
        input |= INPUT_UP;
    }
    if keys.any_just_pressed([ArrowUp, KeyW]) {
        input |= INPUT_UP;
    }
    if keys.any_just_pressed([ArrowDown, KeyS]) {
        input |= INPUT_DOWN;
    }
    if keys.any_just_pressed([ArrowLeft, KeyA]) {
        input |= INPUT_LEFT;
    }
    if keys.any_just_pressed([ArrowRight, KeyD]) {
        input |= INPUT_RIGHT;
    }
    if keys.just_pressed(Space) {
        input |= INPUT_FIRE
    }

    input
}

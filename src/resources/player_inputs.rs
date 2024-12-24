use bevy::{
    input::ButtonInput,
    math::{Vec2, VectorSpace},
    prelude::KeyCode,
};

#[derive(Clone, Copy)]
pub enum PlayerInputCode {
    Up,
    Down,
    Left,
    Right,
    Snapshot,
}

impl PlayerInputCode {
    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            b if b & INPUT_UP != 0 => Some(PlayerInputCode::Up),
            b if b & INPUT_DOWN != 0 => Some(PlayerInputCode::Down),
            b if b & INPUT_LEFT != 0 => Some(PlayerInputCode::Left),
            b if b & INPUT_RIGHT != 0 => Some(PlayerInputCode::Right),
            b if b & INPUT_SNAPSHOT != 0 => Some(PlayerInputCode::Snapshot),
            _ => None,
        }
    }

    pub fn from_keys(keys: &ButtonInput<KeyCode>) -> Option<Self> {
        use KeyCode::*;

        [
            (ArrowUp, PlayerInputCode::Up),
            (ArrowDown, PlayerInputCode::Down),
            (ArrowLeft, PlayerInputCode::Left),
            (ArrowRight, PlayerInputCode::Right),
        ]
        .iter()
        .find(|(key, _)| keys.pressed(*key))
        .map(|(_, dir)| *dir)
        .or_else(|| keys.just_released(KeyP).then(|| PlayerInputCode::Snapshot))
    }

    pub fn to_bits(&self) -> u8 {
        match self {
            PlayerInputCode::Up => INPUT_UP,
            PlayerInputCode::Down => INPUT_DOWN,
            PlayerInputCode::Left => INPUT_LEFT,
            PlayerInputCode::Right => INPUT_RIGHT,
            PlayerInputCode::Snapshot => INPUT_SNAPSHOT,
        }
    }

    pub fn to_vec2(&self) -> Vec2 {
        match self {
            PlayerInputCode::Up => Vec2::Y,
            PlayerInputCode::Down => Vec2::NEG_Y,
            PlayerInputCode::Left => Vec2::NEG_X,
            PlayerInputCode::Right => Vec2::X,
            PlayerInputCode::Snapshot => Vec2::ZERO,
        }
    }
}

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
const INPUT_SNAPSHOT: u8 = 1 << 7;
// const INPUT_FIRE: u8 = 1 << 4;

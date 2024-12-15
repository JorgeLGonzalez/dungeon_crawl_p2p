use bevy::{input::ButtonInput, math::Vec2, prelude::KeyCode};

#[derive(Clone, Copy)]
pub enum InputDirection {
    Up,
    Down,
    Left,
    Right,
}

impl InputDirection {
    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            b if b & INPUT_UP != 0 => Some(InputDirection::Up),
            b if b & INPUT_DOWN != 0 => Some(InputDirection::Down),
            b if b & INPUT_LEFT != 0 => Some(InputDirection::Left),
            b if b & INPUT_RIGHT != 0 => Some(InputDirection::Right),
            _ => None,
        }
    }

    pub fn from_keys(keys: &ButtonInput<KeyCode>) -> Option<Self> {
        use KeyCode::*;

        [
            (ArrowUp, InputDirection::Up),
            (ArrowDown, InputDirection::Down),
            (ArrowLeft, InputDirection::Left),
            (ArrowRight, InputDirection::Right),
        ]
        .iter()
        .find(|(key, _)| keys.pressed(*key))
        .map(|(_, dir)| *dir)
    }

    pub fn to_bits(&self) -> u8 {
        match self {
            InputDirection::Up => INPUT_UP,
            InputDirection::Down => INPUT_DOWN,
            InputDirection::Left => INPUT_LEFT,
            InputDirection::Right => INPUT_RIGHT,
        }
    }

    pub fn to_vec2(&self) -> Vec2 {
        match self {
            InputDirection::Up => Vec2::Y,
            InputDirection::Down => Vec2::NEG_Y,
            InputDirection::Left => Vec2::NEG_X,
            InputDirection::Right => Vec2::X,
        }
    }
}

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;
// const INPUT_FIRE: u8 = 1 << 4;

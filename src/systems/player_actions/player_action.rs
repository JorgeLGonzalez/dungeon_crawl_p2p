use bevy::{input::ButtonInput, math::Vec2, prelude::KeyCode};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlayerAction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    #[default]
    None = 0,
    Snapshot = 100,
}

impl PlayerAction {
    /// Return the direction for a move action.
    pub fn move_direction(&self) -> Option<Vec2> {
        match self {
            PlayerAction::Up => Some(Vec2::Y),
            PlayerAction::Down => Some(Vec2::NEG_Y),
            PlayerAction::Left => Some(Vec2::NEG_X),
            PlayerAction::Right => Some(Vec2::X),
            _ => None,
        }
    }
}

/// Convert from u8 which is how the action is encoded for sharing via GGRS
impl From<u8> for PlayerAction {
    fn from(value: u8) -> Self {
        match value {
            1 => PlayerAction::Up,
            2 => PlayerAction::Down,
            3 => PlayerAction::Left,
            4 => PlayerAction::Right,
            100 => PlayerAction::Snapshot,

            _ => PlayerAction::None,
        }
    }
}

impl From<&ButtonInput<KeyCode>> for PlayerAction {
    fn from(keys: &ButtonInput<KeyCode>) -> Self {
        use KeyCode::*;

        [
            (ArrowUp, PlayerAction::Up),
            (ArrowDown, PlayerAction::Down),
            (ArrowLeft, PlayerAction::Left),
            (ArrowRight, PlayerAction::Right),
        ]
        .iter()
        .find(|(key, _)| keys.pressed(*key))
        .map(|(_, dir)| *dir)
        .or_else(|| keys.just_released(KeyP).then_some(PlayerAction::Snapshot))
        .unwrap_or(PlayerAction::None)
    }
}

/// Convert into u8 for use as LocalInputs to be shared via GGRS
impl Into<u8> for PlayerAction {
    fn into(self) -> u8 {
        self as u8
    }
}

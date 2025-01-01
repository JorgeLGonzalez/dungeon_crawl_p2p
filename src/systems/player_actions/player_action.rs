use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlayerAction {
    Move(MoveDirection),
    #[default]
    None,
    Snapshot,
    StopMoving,
}

/// Convert from u8 which is how the action is encoded for sharing via GGRS
impl From<u8> for PlayerAction {
    fn from(value: u8) -> Self {
        match value {
            1 => PlayerAction::Move(MoveDirection::Up),
            2 => PlayerAction::Move(MoveDirection::Down),
            3 => PlayerAction::Move(MoveDirection::Left),
            4 => PlayerAction::Move(MoveDirection::Right),
            5 => PlayerAction::StopMoving,
            100 => PlayerAction::Snapshot,

            _ => PlayerAction::None,
        }
    }
}

impl From<&ButtonInput<KeyCode>> for PlayerAction {
    fn from(keys: &ButtonInput<KeyCode>) -> Self {
        MOVEMENT_KEYS
            .iter()
            .find(|(key, _)| keys.pressed(*key))
            .map(|(_, dir)| *dir)
            .or_else(|| {
                MOVEMENT_KEYS
                    .iter()
                    .find(|(key, _)| keys.just_released(*key))
                    .map(|_| PlayerAction::StopMoving)
            })
            .or_else(|| {
                keys.just_released(KeyCode::KeyP)
                    .then_some(PlayerAction::Snapshot)
            })
            .unwrap_or(PlayerAction::None)
    }
}

/// Convert into u8 for use as LocalInputs to be shared via GGRS
impl Into<u8> for PlayerAction {
    fn into(self) -> u8 {
        match self {
            PlayerAction::Move(MoveDirection::Up) => 1,
            PlayerAction::Move(MoveDirection::Down) => 2,
            PlayerAction::Move(MoveDirection::Left) => 3,
            PlayerAction::Move(MoveDirection::Right) => 4,
            PlayerAction::StopMoving => 5,
            PlayerAction::Snapshot => 100,
            PlayerAction::None => 0,
        }
    }
}

const MOVEMENT_KEYS: [(KeyCode, PlayerAction); 4] = [
    (KeyCode::ArrowUp, PlayerAction::Move(MoveDirection::Up)),
    (KeyCode::ArrowDown, PlayerAction::Move(MoveDirection::Down)),
    (KeyCode::ArrowLeft, PlayerAction::Move(MoveDirection::Left)),
    (
        KeyCode::ArrowRight,
        PlayerAction::Move(MoveDirection::Right),
    ),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDirection {
    pub fn to_ivec2(&self) -> IVec2 {
        match self {
            MoveDirection::Up => IVec2::Y,
            MoveDirection::Down => IVec2::NEG_Y,
            MoveDirection::Left => IVec2::NEG_X,
            MoveDirection::Right => IVec2::X,
        }
    }
}

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlayerAction {
    GrabItem,
    Move(MoveDirection),
    #[default]
    None,
    /// Show the full dungeon map
    RevealDungeonCheat,
    Snapshot,
    StopMoving,
    ZoomIn,
    UseItem1,
    UseItem2,
    UseItem3,
    UseItem4,
    UseItem5,
    UseItem6,
    UseItem7,
    UseItem8,
    UseItem9,
    ZoomOut,
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
            6 => PlayerAction::GrabItem,
            11 => PlayerAction::UseItem1,
            12 => PlayerAction::UseItem2,
            13 => PlayerAction::UseItem3,
            14 => PlayerAction::UseItem4,
            15 => PlayerAction::UseItem5,
            16 => PlayerAction::UseItem6,
            17 => PlayerAction::UseItem7,
            18 => PlayerAction::UseItem8,
            19 => PlayerAction::UseItem9,
            50 => PlayerAction::ZoomIn,
            51 => PlayerAction::ZoomOut,
            100 => PlayerAction::Snapshot,
            101 => PlayerAction::RevealDungeonCheat,

            _ => PlayerAction::None,
        }
    }
}

impl From<&ButtonInput<KeyCode>> for PlayerAction {
    fn from(keys: &ButtonInput<KeyCode>) -> Self {
        use KeyCode::*;
        use PlayerAction::*;

        MOVEMENT_KEYS
            .iter()
            .find(|(key, _)| keys.pressed(*key))
            .map(|(_, dir)| *dir)
            .or_else(|| {
                MOVEMENT_KEYS
                    .iter()
                    .find(|(key, _)| keys.just_released(*key))
                    .map(|_| StopMoving)
            })
            .or_else(|| keys.pressed(Digit1).then_some(UseItem1))
            .or_else(|| keys.pressed(Digit2).then_some(UseItem2))
            .or_else(|| keys.pressed(Digit3).then_some(UseItem3))
            .or_else(|| keys.pressed(Digit4).then_some(UseItem4))
            .or_else(|| keys.pressed(Digit5).then_some(UseItem5))
            .or_else(|| keys.pressed(Digit6).then_some(UseItem6))
            .or_else(|| keys.pressed(Digit7).then_some(UseItem7))
            .or_else(|| keys.pressed(Digit8).then_some(UseItem8))
            .or_else(|| keys.pressed(Digit9).then_some(UseItem9))
            .or_else(|| keys.pressed(KeyG).then_some(GrabItem))
            .or_else(|| keys.pressed(KeyM).then_some(RevealDungeonCheat))
            .or_else(|| keys.just_released(KeyP).then_some(Snapshot))
            .or_else(|| {
                (keys.just_released(Equal) && keys.any_pressed([ShiftLeft, ShiftRight]))
                    .then_some(ZoomIn)
            })
            .or_else(|| {
                (keys.just_released(Minus) && keys.any_pressed([ShiftLeft, ShiftRight]))
                    .then_some(ZoomOut)
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
            PlayerAction::GrabItem => 6,
            PlayerAction::RevealDungeonCheat => 101,
            PlayerAction::StopMoving => 5,
            PlayerAction::Snapshot => 100,
            PlayerAction::UseItem1 => 11,
            PlayerAction::UseItem2 => 12,
            PlayerAction::UseItem3 => 13,
            PlayerAction::UseItem4 => 14,
            PlayerAction::UseItem5 => 15,
            PlayerAction::UseItem6 => 16,
            PlayerAction::UseItem7 => 17,
            PlayerAction::UseItem8 => 18,
            PlayerAction::UseItem9 => 19,
            PlayerAction::ZoomIn => 50,
            PlayerAction::ZoomOut => 51,
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

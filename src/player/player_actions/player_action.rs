use crate::hud::{InventoryItem, InventoryItemBundle};
use bevy::prelude::*;

pub type PickedItemQuery<'w, 's, 'i, 't> =
    Query<'w, 's, (&'i Interaction, &'t Text), (With<InventoryItem>, Changed<Interaction>)>;

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
    UseItem(u8),
    ZoomOut,
}

impl PlayerAction {
    /// Determine [`PlayerAction`] based on keyboard or mouse click input. The
    /// [`PickedItemQuery`] detects mouse clicks on a HUD UI [`InventoryItem`].
    pub fn new(keys: &mut ButtonInput<KeyCode>, picked_items: &PickedItemQuery) -> Self {
        picked_items
            .iter()
            .find(|(interaction, _)| matches!(interaction, Interaction::Pressed))
            .map(|(_, text)| InventoryItemBundle::index_from_text(text))
            .map(|idx| PlayerAction::UseItem(idx))
            .unwrap_or_else(|| PlayerAction::from(keys))
    }
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
            v if v >= 10 && v <= 18 => PlayerAction::UseItem(v - 10),
            50 => PlayerAction::ZoomIn,
            51 => PlayerAction::ZoomOut,
            100 => PlayerAction::Snapshot,
            101 => PlayerAction::RevealDungeonCheat,

            _ => PlayerAction::None,
        }
    }
}

impl From<&mut ButtonInput<KeyCode>> for PlayerAction {
    fn from(keys: &mut ButtonInput<KeyCode>) -> Self {
        use KeyCode::*;
        use PlayerAction::*;

        /// See "Other Key Inputs" in README for rationale.
        fn single_press(
            keys: &mut ButtonInput<KeyCode>,
            key: KeyCode,
            item: PlayerAction,
        ) -> Option<PlayerAction> {
            keys.pressed(key).then(|| {
                keys.reset(key);
                item
            })
        }

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
            .or_else(|| single_press(keys, Digit1, UseItem(0)))
            .or_else(|| single_press(keys, Digit2, UseItem(1)))
            .or_else(|| single_press(keys, Digit3, UseItem(2)))
            .or_else(|| single_press(keys, Digit4, UseItem(3)))
            .or_else(|| single_press(keys, Digit5, UseItem(4)))
            .or_else(|| single_press(keys, Digit6, UseItem(5)))
            .or_else(|| single_press(keys, Digit7, UseItem(6)))
            .or_else(|| single_press(keys, Digit8, UseItem(7)))
            .or_else(|| single_press(keys, Digit9, UseItem(8)))
            .or_else(|| single_press(keys, KeyG, GrabItem))
            .or_else(|| single_press(keys, KeyM, RevealDungeonCheat))
            .or_else(|| single_press(keys, KeyP, Snapshot))
            .or_else(|| {
                keys.any_pressed([ShiftLeft, ShiftRight])
                    .then(|| {
                        single_press(keys, Equal, ZoomIn)
                            .or_else(|| single_press(keys, Minus, ZoomOut))
                    })
                    .flatten()
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
            PlayerAction::UseItem(v) => 10 + v,
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

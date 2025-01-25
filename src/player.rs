mod camera {
    mod follow_player;
    mod setup_camera;

    pub(super) use follow_player::follow_with_camera;
    pub(super) use setup_camera::setup_camera;

    use super::components::*;
    use super::local_player::*;
}
mod components;
mod events;
mod local_player;
mod player_actions {
    mod attack_monster;
    mod do_player_action;
    mod grab_item;
    mod handle_move_intent;
    mod item_grabber;
    mod move_intent_handler;
    mod move_player;
    mod player_action;
    mod read_player_inputs;
    mod stop_moving;
    mod tick_move_throttle;
    mod use_item;

    pub(super) use attack_monster::attack_monster;
    pub(super) use do_player_action::do_player_action;
    pub(super) use grab_item::grab_item;
    pub(super) use handle_move_intent::handle_move_intent;
    pub(super) use move_player::move_player;
    pub(super) use player_action::PlayerAction;
    pub(super) use read_player_inputs::read_player_inputs;
    pub(super) use stop_moving::stop_moving;
    pub(super) use tick_move_throttle::tick_move_throttle;
    pub(super) use use_item::use_item;

    use super::components::*;
    use super::events::*;
    use item_grabber::*;
    use move_intent_handler::*;
}
mod plugin;
mod spawn_players;

pub use components::{Inventory, MoveThrottle, Obstacle, Player, PlayerCamera, PlayerId};
pub use events::InventoryUpdatedEvent;
pub use local_player::{LocalPlayer, PlayersQuery};
pub use plugin::{PlayerCoreSet, PlayerPlugin};

// use crate::config::*;

pub mod config;
mod dungeon_map;
mod player_inputs;

use bevy::prelude::Resource;
use bevy_matchbox::{
    prelude::{PeerId, SingleChannel},
    MatchboxSocket,
};
pub use dungeon_map::{DungeonMap, DungeonPosition, RandomRoomsBuilder, TileType};
pub use player_inputs::InputDirection;
use rand::{thread_rng, RngCore};

#[derive(Resource)]
pub struct MatchboxSocketResource(pub MatchboxSocket<SingleChannel>);

/// We create the session seed from the socket and peer identifiers so that it
/// is random, but the same for both players. This way the random generator
/// generates the same random numbers for both players so our dungeons (etc) will
/// be the same for both.
#[derive(Clone, Copy, Resource)]
pub struct SessionSeed(u64);

impl SessionSeed {
    /// For P2P sessions
    pub fn from_socket(socket: &mut MatchboxSocket<SingleChannel>) -> Self {
        fn xor(id: PeerId) -> u64 {
            let pair = id.0.as_u64_pair();
            pair.0 ^ pair.1
        }

        let socket_id = xor(socket.id().expect("No peer ID!"));
        let seed = socket
            .connected_peers()
            .map(xor)
            .fold(socket_id, |acc, id| acc ^ id);

        Self(seed)
    }

    /// For single-player sessions
    pub fn new() -> Self {
        Self(thread_rng().next_u64())
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

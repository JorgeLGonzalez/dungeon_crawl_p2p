use bevy::prelude::Resource;
use bevy_matchbox::{prelude::PeerId, MatchboxSocket};
use rand::{distributions::uniform::SampleUniform, prelude::*, Error};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::ops::Range;

/// We seed the random number generator so that in a multi-player game the random
/// numbers generated are exactly the same for both players. This means we do
/// not have to keep random stuff in sync as it is deterministic.
#[derive(Clone, Debug, Resource)]
pub struct RandomGenerator {
    rng: Xoshiro256PlusPlus,
}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {
            rng: Xoshiro256PlusPlus::seed_from_u64(thread_rng().next_u64()),
        }
    }

    pub fn new_for_p2p(socket: &mut MatchboxSocket) -> Self {
        fn xor(id: PeerId) -> u64 {
            let pair = id.0.as_u64_pair();
            pair.0 ^ pair.1
        }

        let socket_id = xor(socket.id().expect("No peer ID!"));
        let seed = socket
            .connected_peers()
            .map(xor)
            .fold(socket_id, |acc, id| acc ^ id);

        Self {
            rng: Xoshiro256PlusPlus::seed_from_u64(seed),
        }
    }

    pub fn gen_range<T>(&mut self, range: Range<T>) -> T
    where
        T: PartialOrd + SampleUniform,
    {
        self.rng.gen_range(range)
    }
}

impl RngCore for RandomGenerator {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}

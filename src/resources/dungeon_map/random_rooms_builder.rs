use super::{dungeon_map::DungeonMap, room::Room};
use crate::resources::{config::*, TileType};
use bevy::log::info;
use rand::prelude::*;

pub struct RandomRoomsBuilder {
    map: DungeonMap,
    rooms: Vec<Room>,
}

impl RandomRoomsBuilder {
    pub fn build() -> DungeonMap {
        let mut builder = Self {
            map: DungeonMap::new(),
            rooms: vec![],
        };

        builder.create_rooms();

        builder.map
    }

    fn create_rooms(&mut self) {
        let mut rng = thread_rng();

        while self.rooms.len() < NUM_ROOMS {
            let room = self.create_room(&mut rng);

            if !self.rooms.iter().any(|r| r.overlaps(&room)) {
                room.tile_positions()
                    .filter(|pos| self.map.is_valid_position(pos))
                    .collect::<Vec<_>>()
                    .iter()
                    .for_each(|pos| {
                        self.map.set_tile_type(pos, TileType::Floor);
                    });

                self.rooms.push(room);
                info!("Room {} of {NUM_ROOMS} generated", self.rooms.len());
            } else {
                info!("Throwing away overlapping room");
            }
        }
    }

    fn create_room(&self, rng: &mut ThreadRng) -> Room {
        const X_MAX: isize = (MAP_WIDTH as isize) / 2;
        const X_MIN: isize = -X_MAX;
        const Y_MAX: isize = (MAP_HEIGHT as isize) / 2;
        const Y_MIN: isize = -Y_MAX;

        Room::new(
            rng.gen_range(X_MIN..X_MAX),
            rng.gen_range(Y_MIN..Y_MAX),
            rng.gen_range(2..ROOM_MAX_WIDTH),
            rng.gen_range(2..ROOM_MAX_HEIGHT),
        )
    }
}

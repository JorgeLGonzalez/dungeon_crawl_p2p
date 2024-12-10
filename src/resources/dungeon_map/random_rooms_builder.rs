use super::{dungeon_map::DungeonMap, room::Room};
use crate::resources::{
    config::*, dungeon_map::dungeon_position::DungeonPosition, SessionSeed, TileType,
};
use bevy::log::info;
use rand::prelude::*;
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct RandomRoomsBuilder {
    map: DungeonMap,
    rooms: Vec<Room>,
}

impl RandomRoomsBuilder {
    pub fn build(session_seed: SessionSeed) -> DungeonMap {
        let mut builder = Self {
            map: DungeonMap::new(),
            rooms: vec![],
        };

        let mut rng = Xoshiro256PlusPlus::seed_from_u64(session_seed.0);

        builder.create_rooms(&mut rng);
        builder.build_corridors(&mut rng);

        builder.map
    }

    fn build_corridors(&mut self, rng: &mut Xoshiro256PlusPlus) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.gen_range(0..2) == 1 {
                self.tunnel_horizontally(prev.x, new.x, prev.y);
                self.tunnel_vertically(prev.y, new.y, new.x);
            } else {
                self.tunnel_vertically(prev.y, new.y, prev.x);
                self.tunnel_horizontally(prev.x, new.x, new.y);
            }
        }
    }

    fn create_rooms(&mut self, rng: &mut Xoshiro256PlusPlus) {
        while self.rooms.len() < NUM_ROOMS {
            let room = self.create_room(rng);

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

    fn create_room(&self, rng: &mut Xoshiro256PlusPlus) -> Room {
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

    fn tunnel_horizontally(&mut self, x1: isize, x2: isize, y: isize) {
        use std::cmp::{max, min};
        (min(x1, x2)..=max(x1, x2))
            .map(|x| DungeonPosition::new(x, y))
            .filter(|pos| self.map.is_valid_position(pos))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|pos| {
                self.map.set_tile_type(pos, TileType::Floor);
            });
    }

    fn tunnel_vertically(&mut self, y1: isize, y2: isize, x: isize) {
        use std::cmp::{max, min};
        (min(y1, y2)..=max(y1, y2))
            .map(|y| DungeonPosition::new(x, y))
            .filter(|pos| self.map.is_valid_position(pos))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|pos| {
                self.map.set_tile_type(pos, TileType::Floor);
            });
    }
}

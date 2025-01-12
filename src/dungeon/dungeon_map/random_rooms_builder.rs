use super::{
    super::{DungeonPosition, TileType},
    dungeon_map::DungeonMap,
    room::Room,
};
use crate::resources::{
    config::{self, *},
    RandomGenerator,
};
use bevy::log::info;
use rand::prelude::*;

pub struct RandomRoomsBuilder {
    map: DungeonMap,
    rooms: Vec<Room>,
}

impl RandomRoomsBuilder {
    pub fn build(rng: &mut RandomGenerator) -> DungeonMap {
        let mut builder = Self {
            map: DungeonMap::new(),
            rooms: vec![],
        };

        builder.create_rooms(rng);
        builder.build_corridors(rng);
        builder.add_player_starting_positions();
        builder.add_monster_starting_positions(rng);

        builder.map
    }

    fn add_monster_starting_positions(&mut self, rng: &mut RandomGenerator) {
        self.map.monster_starting_positions = self
            .map
            .spawnable_positions()
            .choose_multiple(rng, config::NUM_MONSTERS)
    }

    fn add_player_starting_positions(&mut self) {
        self.map
            .player_starting_positions
            .push(self.rooms[0].center());
        if config::GAME_MODE != GameMode::SinglePlayer {
            self.map
                .player_starting_positions
                .push(self.rooms[1].center());
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomGenerator) {
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

    fn create_rooms(&mut self, rng: &mut RandomGenerator) {
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

                info!(
                    "Room {} of {NUM_ROOMS} generated: {room:?}",
                    self.rooms.len()
                );
                self.rooms.push(room);
            } else {
                info!("Throwing away overlapping room");
            }
        }
    }

    fn create_room(&self, rng: &mut RandomGenerator) -> Room {
        const X_MAX: isize = (MAP_WIDTH / 2 - ROOM_MAX_WIDTH - 1) as isize;
        const X_MIN: isize = -((MAP_WIDTH / 2) as isize) + 1;
        const Y_MAX: isize = ((MAP_HEIGHT / 2) - ROOM_MAX_HEIGHT - 1) as isize;
        const Y_MIN: isize = -((MAP_HEIGHT / 2) as isize) + 1;

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

use super::*;
use crate::prelude::*;

pub struct RandomRoomsBuilder {
    map: DungeonMap,
    rooms: Vec<Room>,
}

impl RandomRoomsBuilder {
    pub fn build(level: usize, rng: &mut RandomGenerator) -> DungeonMap {
        info!("Building random rooms dungeon.");
        Self {
            map: DungeonMap::new(level),
            rooms: vec![],
        }
        .create_rooms(rng)
        .build_corridors(rng)
        .set_center()
        .add_player_starting_positions()
        .add_items(rng)
        .add_monster_starting_positions(rng)
        .map
    }

    fn add_items(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.add_items(NUM_ITEMS, rng);

        self
    }

    fn add_monster_starting_positions(mut self, rng: &mut RandomGenerator) -> Self {
        self.map.add_monsters(NUM_MONSTERS, rng);

        self
    }

    fn add_player_starting_positions(mut self) -> Self {
        self.map
            .player_starting_positions
            .push(self.rooms[0].center());
        if config::GAME_MODE != GameMode::SinglePlayer {
            self.map
                .player_starting_positions
                .push(self.rooms[1].center());
        }

        self
    }

    fn build_corridors(mut self, rng: &mut RandomGenerator) -> Self {
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

        self
    }

    fn create_rooms(mut self, rng: &mut RandomGenerator) -> Self {
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

        self
    }

    fn create_room(&self, rng: &mut RandomGenerator) -> Room {
        const ROOM_X_MAX: isize = X_MAX - (ROOM_MAX_WIDTH as isize);
        const ROOM_X_MIN: isize = X_MIN + 1;
        const ROOM_Y_MAX: isize = Y_MAX - (ROOM_MAX_HEIGHT as isize);
        const ROOM_Y_MIN: isize = Y_MIN + 1;

        Room::new(
            rng.gen_range(ROOM_X_MIN..ROOM_X_MAX),
            rng.gen_range(ROOM_Y_MIN..ROOM_Y_MAX),
            rng.gen_range(2..ROOM_MAX_WIDTH),
            rng.gen_range(2..ROOM_MAX_HEIGHT),
        )
    }

    /// Reset the map center to the floor tile nearest the absolute center.
    fn set_center(mut self) -> Self {
        self.map.center = self.map.find_nearest_floor_tile(self.map.center, 1);

        self
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

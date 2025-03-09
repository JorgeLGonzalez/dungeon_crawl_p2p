use super::{DungeonMap, DungeonPosition, TileType};
use bevy::log::info;

pub struct Tunneler<'a> {
    map: &'a mut DungeonMap,
    pos1: DungeonPosition,
    pos2: DungeonPosition,
}

impl<'a> Tunneler<'a> {
    /// Tunnels between two positions in the dungeon map by converting walls to floors.
    /// The tunnel is a simple L shape, first tunneling horizontally and then vertically.
    pub fn tunnel(map: &'a mut DungeonMap, pos1: DungeonPosition, pos2: DungeonPosition) {
        info!("Tunneling from {pos1} to {pos2}");
        let mut tunneler = Tunneler { map, pos1, pos2 };

        let x = tunneler.tunnel_horizontally();
        tunneler.tunnel_vertically(x);
    }

    /// Tunnel horizontally between two positions, returning the x coordinate of
    /// the end position.
    fn tunnel_horizontally(&mut self) -> isize {
        if self.pos1.x == self.pos2.x {
            return self.pos1.x;
        }

        let (x1, x2, y) = if self.pos1.x < self.pos2.x {
            (self.pos1.x, self.pos2.x, self.pos1.y)
        } else {
            (self.pos2.x, self.pos1.x, self.pos2.y)
        };

        for x in x1..=x2 {
            let pos = DungeonPosition::new(x, y);
            if self.map.get_tile_type(&pos) == TileType::Wall {
                info!("Tunneled horizontally at {pos}");
                self.map.set_tile_type(&pos, TileType::Floor);
            }
        }

        x2
    }

    /// Tunnel vertically between two positions at the given x coordinate.
    fn tunnel_vertically(&mut self, x: isize) {
        if self.pos1.y == self.pos2.y {
            return;
        }

        let (y1, y2) = if self.pos1.y < self.pos2.y {
            (self.pos1.y, self.pos2.y)
        } else {
            (self.pos2.y, self.pos1.y)
        };

        for y in y1..=y2 {
            let pos = DungeonPosition::new(x, y);
            if self.map.get_tile_type(&pos) == TileType::Wall {
                info!("Tunneled vertically at {pos}");
                self.map.set_tile_type(&pos, TileType::Floor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tunnel_east() {
        let player_pos = DungeonPosition::new(-3, 0);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_eastbound(player_pos.x, center.x, center.y, &map);
    }

    #[test]
    fn tunnel_west() {
        let player_pos = DungeonPosition::new(3, 0);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_eastbound(center.x, player_pos.x, center.y, &map);
    }

    #[test]
    fn tunnel_north() {
        let player_pos = DungeonPosition::new(0, -3);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_northbound(player_pos.y, center.y, center.x, &map);
    }

    #[test]
    fn tunnel_south() {
        let player_pos = DungeonPosition::new(0, 3);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_northbound(center.y, player_pos.y, center.x, &map);
    }

    #[test]
    fn tunnel_northeast() {
        let player_pos = DungeonPosition::new(-3, -3);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_eastbound(player_pos.x, center.x, player_pos.y, &map);
        assert_northbound(player_pos.y, center.y, center.x, &map);
    }

    #[test]
    fn tunnel_northwest() {
        let player_pos = DungeonPosition::new(3, -3);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_eastbound(center.x, player_pos.x, center.y, &map);
        assert_northbound(player_pos.y, center.y, player_pos.x, &map);
    }

    #[test]
    fn tunnel_southeast() {
        let player_pos = DungeonPosition::new(-5, 5);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_eastbound(player_pos.x, center.x, player_pos.y, &map);
        assert_northbound(center.y, player_pos.y, center.x, &map);
    }

    #[test]
    fn tunnel_southwest() {
        let player_pos = DungeonPosition::new(10, 3);
        let mut map = create_test_map(player_pos);
        let center = map.center;

        Tunneler::tunnel(&mut map, player_pos, center);

        assert_eastbound(center.x, player_pos.x, center.y, &map);
        assert_northbound(center.y, player_pos.y, player_pos.x, &map);
    }

    fn assert_eastbound(x1: isize, x2: isize, y: isize, map: &DungeonMap) {
        for x in x1..=x2 {
            assert_eq!(
                map.get_tile_type(&DungeonPosition::new(x, y)),
                TileType::Floor,
                "Tile at {x},{y} is not floor"
            );
        }
    }

    fn assert_northbound(y1: isize, y2: isize, x: isize, map: &DungeonMap) {
        for y in y1..=y2 {
            assert_eq!(
                map.get_tile_type(&DungeonPosition::new(x, y)),
                TileType::Floor,
                "Tile at {x},{y} is not floor"
            );
        }
    }

    fn create_test_map(player_pos: DungeonPosition) -> DungeonMap {
        let mut map = DungeonMap::new(1);
        let center = map.center;
        map.set_tile_type(&center, TileType::Floor);
        map.set_tile_type(&player_pos, TileType::Floor);

        map
    }
}

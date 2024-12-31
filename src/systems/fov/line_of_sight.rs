use crate::components::{FovRadius, WallTile};
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;

pub type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, With<WallTile>>;

/// Use [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
/// to determine if a wall blocks the line of sight to the given floor tile.
pub struct BresenhamLineOfSight {
    pub pos: IVec2,
    radius_sq: i32,
    wall_set: HashSet<IVec2>,
}

impl BresenhamLineOfSight {
    pub fn new(entity_pos: Vec2, radius: FovRadius, walls: &WallQuery) -> Self {
        Self {
            pos: entity_pos.as_ivec2(),
            radius_sq: (radius * radius) as i32,
            wall_set: Self::create_wall_set(walls),
        }
    }

    pub fn can_see(&self, floor_pos: &IVec2) -> bool {
        if !self.within_radius(floor_pos) {
            return false;
        }

        let mut x = self.pos.x;
        let mut y = self.pos.y;
        let floor_x = floor_pos.x;
        let floor_y = floor_pos.y;

        let x_distance = (floor_x - x).abs();
        let y_distance = (floor_y - y).abs();
        let step_x = if x < floor_x { 1 } else { -1 };
        let step_y = if y < floor_y { 1 } else { -1 };
        let mut error_term = x_distance - y_distance;

        while !(x == floor_x && y == floor_y) {
            if self.wall_set.contains(&IVec2::new(x, y)) {
                return false; // wall obstructs line of sight
            }

            let e2 = 2 * error_term;
            if e2 > -y_distance {
                // step horizontally since we veered off the vertical line
                error_term -= y_distance;
                x += step_x;
            }
            if e2 < x_distance {
                // step vertically since we veered off the horizontal line
                error_term += x_distance;
                y += step_y;
            }
        }

        true // clear line of sight
    }

    fn create_wall_set(walls: &WallQuery) -> HashSet<IVec2> {
        walls
            .iter()
            .map(|t| t.translation.truncate().as_ivec2())
            .collect()
    }

    fn within_radius(&self, floor_pos: &IVec2) -> bool {
        self.pos.distance_squared(*floor_pos) < self.radius_sq
    }
}

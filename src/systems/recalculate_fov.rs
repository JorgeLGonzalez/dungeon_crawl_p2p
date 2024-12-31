use super::player::{FloorQuery, Illuminator};
use crate::{
    components::{FieldOfView, Player, WallTile},
    events::RecalculateFovEvent,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, With<WallTile>>;

pub fn recalculate_fov(
    mut fov_query: Query<&mut FieldOfView, With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor: FloorQuery,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
    walls: WallQuery,
) {
    for event in recalculate_events.read() {
        let entity_pos = event.pos.as_ivec2();
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");
        let radius_sq = (fov.radius * fov.radius) as i32;
        let wall_set = create_wall_set(&walls);

        let visible_tiles: Vec<Entity> = floor
            .iter()
            .map(|(t, tile, _)| (t.translation.truncate().as_ivec2(), tile))
            .filter(|(floor_pos, _)| entity_pos.distance_squared(*floor_pos) < radius_sq)
            .filter(|(floor_pos, _)| is_visible(entity_pos, floor_pos, &wall_set))
            .map(|(_, tile)| tile)
            .collect();

        Illuminator::if_local_player(event.entity, &local_players, &players)
            .with_prior_fov(&fov.visible_tiles)
            .illuminate(&visible_tiles, &mut floor);

        fov.visible_tiles = visible_tiles;
    }
}

fn create_wall_set(walls: &WallQuery) -> HashSet<IVec2> {
    walls
        .iter()
        .map(|t| t.translation.truncate().as_ivec2())
        .collect()
}

/// Use [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
/// to determine if a wall blocks the line of sight to the given floor tile.
fn is_visible(entity_pos: IVec2, floor_pos: &IVec2, wall_set: &HashSet<IVec2>) -> bool {
    let mut x = entity_pos.x;
    let mut y = entity_pos.y;
    let floor_x = floor_pos.x;
    let floor_y = floor_pos.y;

    let x_distance = (floor_x - x).abs();
    let y_distance = (floor_y - y).abs();
    let step_x = if x < floor_x { 1 } else { -1 };
    let step_y = if y < floor_y { 1 } else { -1 };
    let mut error_term = x_distance - y_distance;

    while !(x == floor_x && y == floor_y) {
        if wall_set.contains(&IVec2::new(x, y)) {
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

use super::player::LocalPlayer;
use crate::{
    components::{FieldOfView, FloorTile, Player, WallTile},
    events::RecalculateFovEvent,
    resources::DungeonPosition,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

pub fn recalculate_fov(
    mut fov_query: Query<&mut FieldOfView, With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut tiles: Query<(&Transform, Entity, &mut Sprite), With<FloorTile>>,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
    walls: Query<&Transform, With<WallTile>>,
) {
    for event in recalculate_events.read() {
        let entity_pos = DungeonPosition::from_vec2(event.pos);
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");
        let radius_sq = (fov.radius * fov.radius) as f32;

        let wall_set: HashSet<DungeonPosition> = walls
            .iter()
            .map(|t| DungeonPosition::from_vec2(t.translation.truncate()))
            .collect();

        let visible_tiles: Vec<Entity> = tiles
            .iter()
            .map(|(t, tile, _)| (t.translation.truncate(), tile))
            .filter(|(tile_pos, _)| event.pos.distance_squared(*tile_pos) < radius_sq)
            .filter(|(floor_pos, _)| {
                is_visible(
                    entity_pos,
                    DungeonPosition::from_vec2(*floor_pos),
                    &wall_set,
                )
            })
            .map(|(_, tile)| tile)
            .collect();

        let is_local_player = players
            .get(event.entity)
            .is_ok_and(|player| LocalPlayer::is_local(player, &local_players));
        if is_local_player {
            let mut prior_set: HashSet<Entity> = fov.visible_tiles.iter().map(|e| *e).collect();

            visible_tiles.iter().for_each(|tile| {
                if prior_set.contains(tile) {
                    prior_set.remove(tile);
                } else {
                    let (.., mut sprite) = tiles.get_mut(*tile).expect("Inconceivable!");
                    sprite.color = Color::srgb(0.9, 0.3, 0.5);
                }
            });

            prior_set.iter().for_each(|tile| {
                let (.., mut sprite) = tiles.get_mut(*tile).expect("Inconceivable!");
                sprite.color = Color::srgb(0.5, 0.3, 0.5);
            });
        }

        fov.visible_tiles = visible_tiles;
    }
}

/// Use the [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
/// to determine if a wall blocks the line of sight to the given floor tile.
fn is_visible(
    entity_pos: DungeonPosition,
    floor_pos: DungeonPosition,
    wall_set: &HashSet<DungeonPosition>,
) -> bool {
    let mut entity_x = entity_pos.x;
    let mut entity_y = entity_pos.y;
    let floor_x = floor_pos.x;
    let floor_y = floor_pos.y;

    let x_distance = (floor_x - entity_x).abs();
    let y_distance = (floor_y - entity_y).abs();
    let step_x = if entity_x < floor_x { 1 } else { -1 };
    let step_y = if entity_y < floor_y { 1 } else { -1 };
    let mut error_term = x_distance - y_distance;

    while !(entity_x == floor_x && entity_y == floor_y) {
        if wall_set.contains(&DungeonPosition::new(entity_x, entity_y)) {
            return false; // wall obstructs line of sight
        }

        let e2 = 2 * error_term;
        if e2 > -y_distance {
            // step horizontally since we veered off the vertical line
            error_term -= y_distance;
            entity_x += step_x;
        }
        if e2 < x_distance {
            // step vertically since we veered off the horizontal line
            error_term += x_distance;
            entity_y += step_y;
        }
    }

    true // clear line of sight
}

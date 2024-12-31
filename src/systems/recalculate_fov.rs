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
    let total_tiles = tiles.iter().count();

    for event in recalculate_events.read() {
        let entity_pos = DungeonPosition::from_vec2(event.pos);
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");

        let wall_set: HashSet<DungeonPosition> = walls
            .iter()
            .map(|t| DungeonPosition::from_vec2(t.translation.truncate()))
            .collect();

        let visible_tiles: Vec<Entity> = tiles
            .iter()
            .map(|(t, tile, _)| (t.translation.truncate(), tile))
            // consider using distance squared
            .filter(|(tile_pos, _)| event.pos.distance(*tile_pos) < fov.radius as f32)
            .filter(|(floor_pos, _)| {
                is_visible(
                    entity_pos,
                    DungeonPosition::from_vec2(*floor_pos),
                    &wall_set,
                )
            })
            .map(|(_, tile)| tile)
            .collect();

        info!(
            "Recalculating FOV for {} at {}. See {} tiles of {total_tiles}",
            event.entity,
            event.pos,
            fov.visible_tiles.len()
        );

        let is_local_player = players
            .get(event.entity)
            .map(|player| LocalPlayer::is_local(player, &local_players))
            .is_ok();
        if is_local_player {
            info!("Lighten visible tiles");

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
/// to determine if the a wall blocks the line of sight to the given floor tile.
fn is_visible(
    entity_pos: DungeonPosition,
    floor_pos: DungeonPosition,
    wall_set: &HashSet<DungeonPosition>,
) -> bool {
    let mut x0 = entity_pos.x;
    let mut y0 = entity_pos.y;
    let x1 = floor_pos.x;
    let y1 = floor_pos.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        let point = DungeonPosition::new(x0, y0);
        if wall_set.contains(&point) {
            return false;
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }

    true
}

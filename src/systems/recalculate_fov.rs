use super::player::LocalPlayer;
use crate::{
    components::{FieldOfView, FloorTile, Player, WallTile},
    events::RecalculateFovEvent,
};
use bevy::{prelude::*, utils::hashbrown::HashSet};
use bevy_ggrs::LocalPlayers;

type FloorQuery<'w, 's, 't, 'r> =
    Query<'w, 's, (&'t Transform, Entity, &'r mut Sprite), With<FloorTile>>;
type WallQuery<'w, 's, 't> = Query<'w, 's, &'t Transform, With<WallTile>>;

pub fn recalculate_fov(
    mut fov_query: Query<&mut FieldOfView, With<FieldOfView>>,
    mut recalculate_events: EventReader<RecalculateFovEvent>,
    mut floor_tiles: FloorQuery,
    local_players: Res<LocalPlayers>,
    players: Query<&Player>,
    walls: WallQuery,
) {
    for event in recalculate_events.read() {
        let entity_pos = event.pos.as_ivec2();
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");
        let radius_sq = (fov.radius * fov.radius) as i32;
        let wall_set = create_wall_set(&walls);

        let visible_tiles: Vec<Entity> = floor_tiles
            .iter()
            .map(|(t, tile, _)| (t.translation.truncate().as_ivec2(), tile))
            .filter(|(floor_pos, _)| entity_pos.distance_squared(*floor_pos) < radius_sq)
            .filter(|(floor_pos, _)| is_visible(entity_pos, floor_pos, &wall_set))
            .map(|(_, tile)| tile)
            .collect();

        light_floor_for_local_player(
            &mut floor_tiles,
            event.entity,
            &visible_tiles,
            &fov.visible_tiles,
            &local_players,
            &players,
        );

        fov.visible_tiles = visible_tiles;
    }
}

fn create_wall_set(walls: &WallQuery) -> HashSet<IVec2> {
    walls
        .iter()
        .map(|t| t.translation.truncate().as_ivec2())
        .collect()
}

fn light_floor_for_local_player(
    floor_tiles: &mut FloorQuery,
    entity: Entity,
    fov: &Vec<Entity>,
    fov_prior: &Vec<Entity>,
    local_players: &LocalPlayers,
    players: &Query<&Player>,
) {
    let is_local_player = players
        .get(entity)
        .is_ok_and(|player| LocalPlayer::is_local(player, &local_players));

    if !is_local_player {
        return;
    }

    let mut prior_set: HashSet<Entity> = fov_prior.iter().map(|e| *e).collect();

    fov.iter().for_each(|tile| {
        if prior_set.contains(tile) {
            prior_set.remove(tile);
        } else {
            let (.., mut sprite) = floor_tiles.get_mut(*tile).expect("Inconceivable!");
            sprite.color = Color::srgb(0.9, 0.3, 0.5);
        }
    });

    prior_set.iter().for_each(|tile| {
        let (.., mut sprite) = floor_tiles.get_mut(*tile).expect("Inconceivable!");
        sprite.color = Color::srgb(0.5, 0.3, 0.5);
    });
}

/// Use [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
/// to determine if a wall blocks the line of sight to the given floor tile.
fn is_visible(entity_pos: IVec2, floor_pos: &IVec2, wall_set: &HashSet<IVec2>) -> bool {
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
        if wall_set.contains(&IVec2::new(entity_x, entity_y)) {
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

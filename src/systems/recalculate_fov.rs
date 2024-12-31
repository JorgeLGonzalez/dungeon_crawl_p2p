use super::player::LocalPlayer;
use crate::{
    components::{FieldOfView, FloorTile, Player, WallTile},
    events::RecalculateFovEvent,
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
        let mut fov = fov_query.get_mut(event.entity).expect("Inconceivable!");

        let visible_tiles: Vec<Entity> = tiles
            .iter()
            .map(|(t, tile, _)| (t.translation.truncate(), tile))
            .filter(|(tile_pos, _)| event.pos.distance(*tile_pos) < fov.radius as f32)
            .map(|(_, tile)| tile)
            // TODO line of sight
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

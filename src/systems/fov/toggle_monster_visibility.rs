use crate::components::{FieldOfView, Monster, Player};
use crate::events::ToggleMonsterVisibilityEvent;
use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_ggrs::LocalPlayers;

pub fn toggle_monster_visibility(
    mut monsters: Query<(&Transform, &mut Visibility), With<Monster>>,
    mut event_reader: EventReader<ToggleMonsterVisibilityEvent>,
    local_players: Res<LocalPlayers>,
    players: Query<(&FieldOfView, &Player)>,
) {
    for event in event_reader.read() {
        for player_id in local_players.0.iter() {
            let player_fov: HashSet<IVec2> = players
                .iter()
                .find(|(_, player)| player.id == *player_id)
                .map(|(fov, _)| fov.visible_tiles.keys().copied().collect())
                .expect("Inconceivable!");

            let (transform, mut visibility) =
                monsters.get_mut(event.monster).expect("Inconceivable!");
            let monster_pos = transform.translation.truncate().as_ivec2();

            let expected_visibility = match player_fov.contains(&monster_pos) {
                false => Visibility::Hidden,
                true => Visibility::Visible,
            };

            if *visibility != expected_visibility {
                info!(
                    "Toggling visibility {visibility:?} for monster at {monster_pos} for player {player_id}",

                );
                visibility.toggle_visible_hidden();
            }
        }
    }
}

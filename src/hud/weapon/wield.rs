use super::WeaponText;
use crate::{
    player::{LocalPlayer, WeaponWieldedEvent},
    prelude::*,
};
use bevy_ggrs::LocalPlayers;

pub fn wield_weapon(
    mut weapon_text: Query<&mut Text, With<WeaponText>>,
    mut wield_events: EventReader<WeaponWieldedEvent>,
    local_players: Res<LocalPlayers>,
) {
    wield_events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.player_id, &local_players))
        .for_each(|event| {
            info!(
                "Player {} wields weapon {}",
                event.player_id,
                event.weapon.label()
            );
            weapon_text.single_mut().0 = format!("Weapon: {}", event.weapon.label());
        });
}

use super::super::player::{LocalPlayer, PlayersQuery};
use crate::{
    components::{HealthBar, Player},
    resources::config,
};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn health_bar(
    local_players: Res<LocalPlayers>,
    players: PlayersQuery,
    mut health_bars: Query<(&mut Transform, &mut Sprite), (With<HealthBar>, Without<Player>)>,
) {
    let player = LocalPlayer::new(&local_players, &players);
    let (mut health_bar, mut bar_sprite) = health_bars.single_mut();
    health_bar.translation.y = player.pos.y + HEALTH_BAR_OFFSET;
    health_bar.translation.x = player.pos.x;

    if let Some(size) = bar_sprite.custom_size.as_mut() {
        size.x = player.health as f32;
    }
}

const HEALTH_BAR_OFFSET: f32 = config::VIEWPORT_HEIGHT * config::CAMERA_SCALE / 2. - 0.5;

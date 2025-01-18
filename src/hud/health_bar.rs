use super::components::{HealthBar, HealthPointsText};
use crate::config;
use crate::player::{LocalPlayer, Player, PlayersQuery};
use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;

pub fn health_bar(
    local_players: Res<LocalPlayers>,
    players: PlayersQuery,
    mut health_bars: Query<&mut Node, (With<HealthBar>, Without<Player>)>,
    mut health_points: Query<&mut Text, With<HealthPointsText>>,
) {
    let player = LocalPlayer::new(&local_players, &players);
    health_bars.single_mut().width =
        Val::Percent(100. * player.health as f32 / config::PLAYER_HEALTH_MAX as f32);
    health_points.single_mut().0 = format!("{}/{}", player.health, config::PLAYER_HEALTH_MAX);
}

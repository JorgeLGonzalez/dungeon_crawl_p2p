use crate::{
    components::{Damage, FieldOfView, Health, Obstacle},
    dungeon::DungeonMap,
    hud::TooltipLabel,
    player::{LocalPlayer, Player},
    resources::config::{self, PLAYER_HEIGHT, PLAYER_WIDTH},
};
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, LocalPlayers};

pub fn spawn_players(
    dungeon: Res<DungeonMap>,
    local_players: Res<LocalPlayers>,
    mut commands: Commands,
) {
    for (player_idx, &player_pos) in dungeon.player_starting_positions.iter().enumerate() {
        let color = match player_idx {
            0 => config::PLAYER_0_COLOR,
            _ => config::PLAYER_1_COLOR,
        };

        let is_remote_player = !LocalPlayer::is_local_player_id(player_idx, &local_players);

        let id = commands
            .spawn((
                Player { id: player_idx },
                Damage(1),
                FieldOfView::new(config::PLAYER_FOV_RADIUS),
                Health::new(config::PLAYER_HEALTH_MAX),
                Obstacle::Player,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..default()
                },
                TooltipLabel(format!("Player {}", player_idx)),
                Transform::from_translation(player_pos.to_vec3(config::PLAYER_Z_LAYER)),
            ))
            .insert_if(Visibility::Hidden, || is_remote_player)
            .add_rollback()
            .id();

        info!(
            "Spawned {} player {player_idx} at {player_pos} [{id}]",
            if is_remote_player { "remote" } else { "local" }
        );
    }
}

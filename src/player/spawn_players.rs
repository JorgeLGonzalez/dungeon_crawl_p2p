use super::components::{Inventory, Obstacle, Player};
use crate::{
    config::{PLAYER_HEIGHT, PLAYER_WIDTH},
    hud::TooltipLabel,
    prelude::*,
};
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_players(dungeon: Res<DungeonMap>, mut commands: Commands, players: Query<&Player>) {
    if !players.is_empty() {
        // Players already spawned, do nothing
        return;
    }

    for (player_idx, &player_pos) in dungeon.player_starting_positions.iter().enumerate() {
        let color = match player_idx {
            0 => config::PLAYER_0_COLOR,
            _ => config::PLAYER_1_COLOR,
        };

        let id = commands
            .spawn((
                Player { id: player_idx },
                Damage(1),
                FieldOfView::new(config::PLAYER_FOV_RADIUS),
                Health::new(config::PLAYER_HEALTH_MAX),
                Inventory::new(),
                Obstacle::Player,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..default()
                },
                TooltipLabel(format!("Player {}", player_idx)),
                Transform::from_translation(player_pos.to_vec3(config::PLAYER_Z_LAYER)),
                Visibility::Hidden,
            ))
            .add_rollback()
            .id();

        info!("Spawned player {player_idx} at {player_pos} [{id}]");
    }
}

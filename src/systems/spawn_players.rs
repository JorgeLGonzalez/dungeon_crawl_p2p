use crate::{
    components::{Damage, FieldOfView, Health, Obstacle, Player},
    dungeon::DungeonMap,
    hud::TooltipLabel,
    resources::config::{self, PLAYER_HEIGHT, PLAYER_WIDTH},
};
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_players(dungeon: Res<DungeonMap>, mut commands: Commands) {
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
                Obstacle::Player,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..default()
                },
                TooltipLabel(format!("Player {}", player_idx)),
                Transform::from_translation(player_pos.to_vec3(config::PLAYER_Z_LAYER)),
            ))
            .add_rollback()
            .id();

        info!("Spawned player {player_idx} at {player_pos} [{id}]");
    }
}

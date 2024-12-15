use crate::{
    components::{Player, PlayerMovement},
    resources::{
        config::{self, PLAYER_HEIGHT, PLAYER_WIDTH},
        DungeonMap,
    },
};
use bevy::{
    color::Color,
    log::info,
    math::Vec2,
    prelude::{Commands, Res, Transform},
    sprite::Sprite,
    utils::default,
};
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_players(dungeon: Res<DungeonMap>, mut commands: Commands) {
    for (player_idx, &player_pos) in dungeon.player_starting_positions.iter().enumerate() {
        let color = match player_idx {
            0 => Color::srgb(0., 0., 1.),
            _ => Color::srgb(0., 1., 0.),
        };

        commands
            .spawn((
                Player { id: player_idx },
                PlayerMovement::default(),
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                    ..default()
                },
                Transform::from_translation(player_pos.to_vec3(config::PLAYER_Z_LAYER)),
            ))
            .add_rollback();

        info!("Spawned player {player_idx} at {player_pos:?}");
    }
}

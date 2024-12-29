use crate::{
    components::{Monster, Obstacle},
    resources::{
        config::{self, TILE_HEIGHT, TILE_WIDTH},
        DungeonMap,
    },
};
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_monsters(dungeon: Res<DungeonMap>, mut commands: Commands) {
    for pos in &dungeon.monster_starting_positions {
        let color = Color::srgb_from_array(Vec3::splat(0.3).to_array());

        commands
            .spawn((
                Monster,
                Obstacle::Monster,
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                    ..default()
                },
                Transform::from_translation(pos.to_vec3(config::MONSTER_Z_LAYER)),
            ))
            .add_rollback();
    }
}

use crate::{
    components::{FieldOfView, Health, LastAction, Monster, Obstacle},
    resources::{
        config::{self, TILE_HEIGHT, TILE_WIDTH},
        DungeonMap,
    },
};
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_monsters(dungeon: Res<DungeonMap>, mut commands: Commands) {
    for pos in &dungeon.monster_starting_positions {
        commands
            .spawn((
                Monster,
                FieldOfView::new(config::MONSTER_FOV_RADIUS),
                Health::new(1),
                LastAction::new(),
                Obstacle::Monster,
                Sprite {
                    color: config::MONSTER_COLOR,
                    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                    ..default()
                },
                Transform::from_translation(pos.to_vec3(config::MONSTER_Z_LAYER)),
            ))
            .add_rollback();
    }
}

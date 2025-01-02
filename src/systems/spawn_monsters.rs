use crate::{
    components::{Damage, FieldOfView, Health, LastAction, Monster, MonsterType, Obstacle},
    resources::{
        config::{self, TILE_HEIGHT, TILE_WIDTH},
        DungeonMap, RandomGenerator,
    },
};
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_monsters(
    dungeon: Res<DungeonMap>,
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
) {
    for pos in &dungeon.monster_starting_positions {
        let (damage, monster_type, health, color) = random_monster(&mut rng);
        commands
            .spawn((
                Monster,
                damage,
                FieldOfView::new(config::MONSTER_FOV_RADIUS),
                health,
                LastAction::new(),
                monster_type,
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

fn random_monster(rng: &mut RandomGenerator) -> (Damage, MonsterType, Health, Color) {
    let monster_type = match rng.gen_range(0..10) {
        0 => MonsterType::Ettin,
        1 => MonsterType::Ogre,
        2 => MonsterType::Orc,
        _ => MonsterType::Goblin,
    };

    let (damage, health, color) = match monster_type {
        MonsterType::Ettin => (Damage(3), Health::new(10), Color::srgb(0.9, 0.1, 0.1)),
        MonsterType::Ogre => (Damage(2), Health::new(2), Color::srgb(0.8, 0.2, 0.2)),
        MonsterType::Orc => (Damage(1), Health::new(2), Color::srgb(0.7, 0.3, 0.3)),
        MonsterType::Goblin => (Damage(1), Health::new(1), Color::srgb(0.6, 0.4, 0.4)),
    };

    (damage, monster_type, health, color)
}

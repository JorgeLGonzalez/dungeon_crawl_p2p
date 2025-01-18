use super::components::{LastAction, Monster, MonsterType};
use crate::{
    components::{Damage, FieldOfView, Health, Obstacle},
    dungeon::DungeonMap,
    hud::TooltipLabel,
    resources::{
        config::{self, TILE_HEIGHT, TILE_WIDTH},
        RandomGenerator,
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
        let (tooltip, damage, monster_type, health, color) = random_monster(&mut rng);
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
                tooltip,
                Transform::from_translation(pos.to_vec3(config::MONSTER_Z_LAYER)),
                Visibility::Hidden,
            ))
            .add_rollback();
    }
}

fn random_monster(rng: &mut RandomGenerator) -> (TooltipLabel, Damage, MonsterType, Health, Color) {
    let monster_type = match rng.gen_range(0..10) {
        0 => MonsterType::Ettin,
        1 => MonsterType::Ogre,
        2 => MonsterType::Orc,
        _ => MonsterType::Goblin,
    };

    let (name, damage, health, color) = match monster_type {
        MonsterType::Ettin => (
            "Ettin",
            Damage(3),
            Health::new(10),
            Color::srgb(0.9, 0.1, 0.1),
        ),
        MonsterType::Ogre => (
            "Ogre",
            Damage(2),
            Health::new(2),
            Color::srgb(0.8, 0.2, 0.2),
        ),
        MonsterType::Orc => ("Orc", Damage(1), Health::new(2), Color::srgb(0.7, 0.3, 0.3)),
        MonsterType::Goblin => (
            "Goblin",
            Damage(1),
            Health::new(1),
            Color::srgb(0.6, 0.4, 0.4),
        ),
    };

    (
        TooltipLabel(format!("{name}: {} hp", health.max)),
        damage,
        monster_type,
        health,
        color,
    )
}

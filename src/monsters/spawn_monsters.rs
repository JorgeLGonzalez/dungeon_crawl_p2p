use super::{LastAction, Monster};
use crate::{
    config::{TILE_HEIGHT, TILE_WIDTH},
    player::Obstacle,
    prelude::*,
};
use bevy_ggrs::AddRollbackCommandExtension;

pub fn spawn_monsters(
    dungeon: Res<DungeonMap>,
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
) {
    for pos in &dungeon.monster_starting_positions {
        let monster = random_monster(&mut rng);
        commands
            .spawn((
                monster,
                monster.damage(),
                FieldOfView::new(config::MONSTER_FOV_RADIUS),
                monster.health(),
                LastAction::new(),
                Obstacle::Monster,
                Sprite {
                    color: monster.color(),
                    custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
                    ..default()
                },
                monster.tooltip(),
                Transform::from_translation(pos.to_vec3(config::MONSTER_Z_LAYER)),
                Visibility::Hidden,
            ))
            .add_rollback();
    }
}

fn random_monster(rng: &mut RandomGenerator) -> Monster {
    match rng.gen_range(0..10) {
        0 => Monster::Ettin,
        1 => Monster::Ogre,
        2 => Monster::Orc,
        _ => Monster::Goblin,
    }
}

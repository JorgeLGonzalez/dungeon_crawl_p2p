use super::{Monster, MonsterBundle};
use crate::{
    common::{DungeonAssets, DungeonData},
    prelude::*,
};
use bevy::utils::hashbrown::HashMap;
use bevy_ggrs::AddRollbackCommandExtension;
use std::iter::repeat;

pub fn spawn_monsters(
    dungeon: Res<DungeonMap>,
    dungeon_assets: Res<DungeonAssets>,
    dungeon_data_assets: Res<Assets<DungeonData>>,
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
) {
    let monster_distribution = create_distribution(dungeon_data_assets.get(&dungeon_assets.data));
    let mut random_monster =
        || monster_distribution[rng.gen_range(0..monster_distribution.len())].clone();

    let stats = dungeon
        .monster_starting_positions()
        .map(|monster_pos| {
            (
                monster_pos.monster.unwrap_or_else(&mut random_monster),
                monster_pos.pos.to_vec2(),
            )
        })
        .map(|(template, pos)| MonsterBundle::new(template, pos))
        .fold(HashMap::new(), |mut acc, monster_bundle| {
            acc.entry(monster_bundle.monster.name().to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);

            commands.spawn(monster_bundle).add_rollback();

            acc
        });

    info!("Spawned monsters: {stats:?}");
}

/// Create a distribution of monster templates based on their frequency so that
/// those with a higher frequency are more likely to be randomly selected.
fn create_distribution(dungeon_data: Option<&DungeonData>) -> Vec<&Monster> {
    dungeon_data
        .expect("Failed to load dungeon data")
        .monsters
        .iter()
        .flat_map(|template| repeat(&template.monster).take(template.frequency))
        .collect()
}

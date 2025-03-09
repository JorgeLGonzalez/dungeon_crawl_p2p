use super::{MagicItem, MagicItemBundle};
use crate::{
    common::{DungeonAssets, DungeonData},
    prelude::*,
};
use bevy::utils::hashbrown::HashMap;
use bevy_ggrs::AddRollbackCommandExtension;
use std::iter::repeat;

pub fn spawn_items(
    dungeon: Res<DungeonMap>,
    dungeon_assets: Res<DungeonAssets>,
    dungeon_data_assets: Res<Assets<DungeonData>>,
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
) {
    let item_distribution = create_distribution(dungeon_data_assets.get(&dungeon_assets.data));
    let mut random_item = || item_distribution[rng.gen_range(0..item_distribution.len())].clone();

    let stats = dungeon
        .item_positions()
        .map(|item_pos| {
            (
                item_pos.item.unwrap_or_else(&mut random_item),
                item_pos.pos.to_vec2(),
            )
        })
        .map(|(item, pos)| MagicItemBundle::new(item, pos))
        .fold(HashMap::new(), |mut acc, item_bundle| {
            acc.entry(item_bundle.item.label())
                .and_modify(|count| *count += 1)
                .or_insert(1);

            commands.spawn(item_bundle).add_rollback();

            acc
        });

    info!("Spawned items: {stats:?}");
}

/// Create a distribution of item templates based on their frequency so that
/// those with a higher frequency are more likely to be randomly selected.
fn create_distribution(dungeon_data: Option<&DungeonData>) -> Vec<MagicItem> {
    dungeon_data
        .expect("Failed to load dungeon data")
        .items
        .iter()
        .flat_map(|template| repeat(template.item).take(template.frequency))
        .collect()
}

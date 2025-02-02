use super::{Grabbable, MagicItemTemplate};
use crate::{
    common::{DungeonAssets, DungeonData},
    hud::TooltipLabel,
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

    let mut stats: HashMap<String, usize> = HashMap::new();

    for pos in &dungeon.item_positions {
        let template = item_distribution[rng.gen_range(0..item_distribution.len())];
        let item = template.to_magic_item();
        commands
            .spawn((
                item,
                Grabbable,
                Sprite {
                    color: template.color(),
                    custom_size: Some(Vec2::new(config::TILE_WIDTH, config::TILE_HEIGHT)),
                    ..default()
                },
                TooltipLabel(item.label()),
                Transform::from_translation(pos.to_vec3(config::ITEM_Z_LAYER)),
                Visibility::Hidden,
            ))
            .add_rollback();

        stats
            .entry(item.label())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    info!("Spawned items: {stats:?}");
}

/// Create a distribution of item templates based on their frequency so that
/// those with a higher frequency are more likely to be randomly selected.
fn create_distribution(dungeon_data: Option<&DungeonData>) -> Vec<&MagicItemTemplate> {
    dungeon_data
        .expect("Failed to load dungeon data")
        .items
        .iter()
        .flat_map(|template| repeat(template).take(template.frequency))
        .collect()
}

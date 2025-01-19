use super::MagicItem;
use crate::prelude::*;

pub fn spawn_items(
    dungeon: Res<DungeonMap>,
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
) {
    for pos in &dungeon.item_positions {
        let item = random_item(&mut rng);
        commands.spawn((
            item,
            Sprite {
                color: item.color(),
                custom_size: Some(Vec2::new(config::TILE_WIDTH, config::TILE_HEIGHT)),
                ..default()
            },
            item.tooltip(),
            Transform::from_translation(pos.to_vec3(config::ITEM_Z_LAYER)),
            // Visibility::Hidden,
        ));
    }
}

fn random_item(rng: &mut RandomGenerator) -> MagicItem {
    match rng.gen_range(0..10) {
        0..3 => MagicItem::HealingPotion,
        _ => MagicItem::HealingPotionWeak,
    }
}

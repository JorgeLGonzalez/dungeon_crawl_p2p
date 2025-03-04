use super::*;
use crate::{player::Obstacle, prelude::*};

pub fn spawn_dungeon(mut commands: Commands, mut rng: ResMut<RandomGenerator>) {
    let mut dungeon = match rng.gen_range(0..3) {
        0 => CellAutomataBuilder::build(rng.as_mut()),
        1 => DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), rng.as_mut()),
        2 => RandomRoomsBuilder::build(rng.as_mut()),
        _ => unreachable!(),
    };

    let mut prefab = PrefabVault::new(PrefabBlueprint::Fortress);
    if let Some(pos) = prefab.determine_location(&dungeon, &mut rng) {
        prefab.create_at(pos, &mut dungeon);
    } else {
        warn!("Unable to find a proper location for the prefab vault!");
    }

    for tile in dungeon.tiles() {
        let sprite = create_sprite(tile.tile_type);
        let transform = Transform::from_translation(tile.pos.into());

        let mut tile_entity = match tile.tile_type {
            TileType::Exit => commands.spawn((ExitTile,)),
            TileType::Floor => commands.spawn((FloorTile,)),
            TileType::Wall => commands.spawn((WallTile, Obstacle::Wall)),
        };

        tile_entity.insert((sprite, transform, Visibility::Hidden));
    }

    commands.insert_resource(dungeon);
}

fn create_sprite(tile_type: TileType) -> Sprite {
    let color = match tile_type {
        TileType::Exit => Color::srgb(1., 1., 1.),
        TileType::Floor => FLOOR_COLOR,
        TileType::Wall => Color::srgb(0., 0., 0.),
    };

    Sprite {
        color,
        custom_size: Some(Vec2::new(TILE_HEIGHT, TILE_WIDTH)),
        ..default()
    }
}

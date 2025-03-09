use super::*;
use crate::{config::ITEM_Z_LAYER, hud::TooltipLabel, player::Obstacle, prelude::*};

pub fn spawn_dungeon(
    mut commands: Commands,
    mut rng: ResMut<RandomGenerator>,
    dungeon: Option<Res<DungeonMap>>,
) {
    let level = dungeon.map_or(1, |dungeon| dungeon.level + 1);
    info!("Spawning dungeon level {level}");

    let mut dungeon = match level {
        1 => RandomRoomsBuilder::build(level, rng.as_mut()),
        2 => DrunkardsWalkBuilder::build(DrunkardsWalkConfig::default(), level, rng.as_mut()),
        3 => CellAutomataBuilder::build(level, rng.as_mut()),
        _ => CellAutomataBuilder::build(level, rng.as_mut()),
    };

    PrefabVault::from(PrefabBlueprint::Fortress).create_in(&mut dungeon, &mut rng);

    for tile in dungeon.tiles() {
        let sprite = create_sprite(tile.tile_type);
        let transform = Transform::from_translation(tile.pos.into());

        let mut tile_entity = match tile.tile_type {
            TileType::Floor => commands.spawn((FloorTile,)),
            TileType::Wall => commands.spawn((WallTile, Obstacle::Wall)),
        };

        tile_entity.insert((sprite, transform, Visibility::Hidden));
    }

    spawn_exit_stairs(&mut commands, &dungeon);

    commands.insert_resource(dungeon);
}

fn create_sprite(tile_type: TileType) -> Sprite {
    let color = match tile_type {
        TileType::Floor => FLOOR_COLOR,
        TileType::Wall => Color::srgb(0., 0., 0.),
    };

    Sprite {
        color,
        custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
        ..default()
    }
}

fn spawn_exit_stairs(commands: &mut Commands, dungeon: &DungeonMap) {
    commands.spawn((
        ExitStairs,
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(TILE_WIDTH, TILE_HEIGHT)),
            ..default()
        },
        TooltipLabel("Exit Stairs".to_string()),
        Transform::from_translation(dungeon.center.to_vec3(ITEM_Z_LAYER)),
        // TODO: hide
        // Visibility::Hidden,
    ));

    info!(
        "Exit stairs at {} on tile type {:?}",
        dungeon.center,
        dungeon.get_tile_type(&dungeon.center)
    );
}

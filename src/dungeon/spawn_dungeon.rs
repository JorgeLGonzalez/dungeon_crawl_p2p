use super::dungeon_map::RandomRoomsBuilder;
use super::{ExitTile, FloorTile, TileType, WallTile};
use crate::config::{self, TILE_HEIGHT, TILE_WIDTH};
use crate::{components::Obstacle, resources::RandomGenerator};
use bevy::prelude::*;

pub fn spawn_dungeon(mut commands: Commands, mut rng: ResMut<RandomGenerator>) {
    let dungeon = RandomRoomsBuilder::build(rng.as_mut());

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
        TileType::Floor => config::FLOOR_COLOR,
        TileType::Wall => Color::srgb(0., 0., 0.),
    };

    Sprite {
        color,
        custom_size: Some(Vec2::new(TILE_HEIGHT, TILE_WIDTH)),
        ..default()
    }
}

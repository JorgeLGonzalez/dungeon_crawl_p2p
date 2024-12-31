use crate::{
    components::{ExitTile, FloorTile, Obstacle, WallTile},
    resources::{
        config::{self, TILE_HEIGHT, TILE_WIDTH},
        RandomGenerator, RandomRoomsBuilder, TileType,
    },
};
use bevy::prelude::*;

pub fn spawn_dungeon(mut commands: Commands, mut rng: ResMut<RandomGenerator>) {
    let dungeon = RandomRoomsBuilder::build(rng.as_mut());

    for tile in dungeon.tiles() {
        let sprite = create_sprite(tile.tile_type);
        let transform = Transform::from_translation(tile.pos.into());

        match tile.tile_type {
            TileType::Exit => commands.spawn((ExitTile, sprite, transform)),
            TileType::Floor => commands.spawn((FloorTile, sprite, transform)),
            TileType::Wall => commands.spawn((WallTile, Obstacle::Wall, sprite, transform)),
        };
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

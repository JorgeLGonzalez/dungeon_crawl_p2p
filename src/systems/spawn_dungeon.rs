use crate::{
    components::{ExitTile, FloorTile, WallTile},
    resources::{
        config::{TILE_HEIGHT, TILE_WIDTH},
        RandomRoomsBuilder, SessionSeed, TileType,
    },
};
use bevy::{
    color::Color,
    math::Vec2,
    prelude::{Commands, Res, Transform},
    sprite::Sprite,
    utils::default,
};

pub fn spawn_dungeon(mut commands: Commands, session_seed: Res<SessionSeed>) {
    let dungeon = RandomRoomsBuilder::build(*session_seed);

    for tile in dungeon.tiles() {
        let sprite = create_sprite(tile.tile_type);
        let transform = Transform::from_translation(tile.pos.into());

        match tile.tile_type {
            TileType::Exit => {
                commands.spawn((ExitTile, sprite, transform));
            }
            TileType::Floor => {
                commands.spawn((FloorTile, sprite, transform));
            }
            TileType::Wall => {
                commands.spawn((WallTile, sprite, transform));
            }
        }
    }

    commands.insert_resource(dungeon);
}

fn create_sprite(tile_type: TileType) -> Sprite {
    let color = match tile_type {
        TileType::Exit => Color::srgb(1., 1., 1.),
        TileType::Floor => Color::srgb(0.5, 0.3, 0.5),
        TileType::Wall => Color::srgb(0., 0., 0.),
    };

    Sprite {
        color,
        custom_size: Some(Vec2::new(TILE_HEIGHT, TILE_WIDTH)),
        ..default()
    }
}

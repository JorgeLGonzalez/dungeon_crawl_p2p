use crate::{
    components::Tile,
    resources::{RandomRoomsBuilder, SessionSeed, TileType},
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
        let color = match tile.tile_type {
            TileType::Exit => Color::srgb(1., 1., 1.),
            TileType::Floor => Color::srgb(0.5, 0.3, 0.5),
            TileType::Wall => Color::srgb(0., 0., 0.),
        };

        commands.spawn((
            Tile,
            Sprite {
                color,
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            Transform::from_translation(tile.pos.into()),
        ));
    }

    commands.insert_resource(dungeon);
}

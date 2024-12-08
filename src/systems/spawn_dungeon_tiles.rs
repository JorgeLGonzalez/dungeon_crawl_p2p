use crate::{
    components::Tile,
    resources::{config, RandomRoomsBuilder, TileType},
};
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn spawn_dungeon_tiles(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config::VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let dungeon = RandomRoomsBuilder::build();

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
}

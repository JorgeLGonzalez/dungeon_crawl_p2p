use crate::{
    components::Tile,
    resources::{config, DungeonMap},
};
use bevy::{prelude::*, render::camera::ScalingMode};

pub fn generate_map(mut commands: Commands) {
    info!("Generating map");

    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: config::VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    let map = DungeonMap::new();

    for (idx, tile) in map.tiles.iter().enumerate() {
        commands.spawn((
            Tile,
            Sprite {
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            Transform::from_translation(map.idx_to_position(idx).extend(10.)),
        ));
    }
}

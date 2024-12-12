use crate::{
    components::Player,
    resources::{config, DungeonMap},
};
use bevy::{
    color::Color,
    math::Vec2,
    prelude::{Commands, Res, Transform},
    sprite::Sprite,
    utils::default,
};

pub fn spawn_players(mut commands: Commands, dungeon: Res<DungeonMap>) {
    for (player_idx, &player_pos) in dungeon.player_starting_positions.iter().enumerate() {
        let color = match player_idx {
            0 => Color::srgb(0., 0., 1.),
            _ => Color::srgb(0., 1., 0.),
        };

        commands.spawn((
            Player { id: player_idx },
            Sprite {
                color,
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            Transform::from_translation(player_pos.into()),
        ));
    }
}

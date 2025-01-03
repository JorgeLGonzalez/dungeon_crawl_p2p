use crate::{
    components::HealthBar,
    resources::config::{self, HUD_Z_LAYER},
};
use bevy::{prelude::*, sprite::Anchor};

pub fn spawn_health_bar(mut commands: Commands) {
    let anchor = Anchor::CenterLeft;
    let custom_size = Some(Vec2::new(config::PLAYER_HEALTH_MAX as f32, 0.5));

    commands
        .spawn((
            HealthBar,
            Sprite {
                anchor,
                color: Color::srgb(0., 0.7, 0.),
                custom_size,
                ..default()
            },
            Transform::from_xyz(0., config::VIEWPORT_HEIGHT, HUD_Z_LAYER + 1.),
        ))
        .with_child((
            Sprite {
                anchor,
                color: Color::srgb(0.5, 0.5, 0.5),
                custom_size,
                ..default()
            },
            Transform::from_xyz(0., 0., -1.),
        ));
}

use crate::{
    components::{HealthBar, HealthPointsText},
    resources::config::{self, HUD_Z_LAYER},
};
use bevy::{prelude::*, sprite::Anchor};

pub fn spawn_health_bar(mut commands: Commands /* assets: Res<AssetServer> */) {
    const BAR_HEIGHT: f32 = 0.5;
    const BAR_WIDTH: f32 = config::PLAYER_HEALTH_MAX as f32;
    let anchor = Anchor::CenterLeft;
    let custom_size = Some(Vec2::new(BAR_WIDTH, BAR_HEIGHT));

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
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    anchor,
                    color: Color::srgb(0.5, 0.5, 0.5),
                    custom_size,
                    ..default()
                },
                Transform::from_xyz(0., 0., -1.),
            ));

            let font_size = 20.;
            let scale = Vec3::splat(BAR_HEIGHT / font_size);
            parent.spawn((
                Text2d::new("Health"),
                TextFont {
                    // font: assets.load("fonts/FiraSans-Bold.ttf"), // works on web not mac (path not found)
                    font_size,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Right),
                Transform {
                    translation: Vec3::new(-1., 0., 0.),
                    scale,
                    ..default()
                },
            ));

            parent.spawn((
                HealthPointsText,
                Text2d::new("10/10"),
                TextFont {
                    font_size,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Left),
                Transform {
                    translation: Vec3::new(BAR_WIDTH + 1., 0., 0.),
                    scale,
                    ..default()
                },
            ));
        });
}

use super::{config, FontAssets, WeaponText};
use crate::{
    player::{LocalPlayer, WeaponWieldedEvent},
    prelude::*,
};
use bevy::render::view::RenderLayers;
use bevy_ggrs::LocalPlayers;

pub fn spawn_weapon_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            BackgroundColor(config::BACKGROUND_COLOR.into()),
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::RowReverse,
                margin: UiRect::all(Val::Px(10.)),
                padding: UiRect::right(Val::Px(20.)),
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                ..default()
            },
            RenderLayers::layer(config::CAMERA_RENDER_LAYER),
        ))
        .with_child((
            WeaponText,
            Text::new("Weapon: None"),
            TextFont {
                font: font_assets.hud_font.clone(),
                font_size: config::TEXT_SIZE,
                ..default()
            },
        ));
}

pub fn wield_weapon(
    mut weapon_text: Query<&mut Text, With<WeaponText>>,
    mut wield_events: EventReader<WeaponWieldedEvent>,
    local_players: Res<LocalPlayers>,
) {
    wield_events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.player_id, &local_players))
        .for_each(|event| {
            info!(
                "Player {} wields weapon {}",
                event.player_id,
                event.weapon.label()
            );
            weapon_text.single_mut().0 = format!("Weapon: {}", event.weapon.label());
        });
}

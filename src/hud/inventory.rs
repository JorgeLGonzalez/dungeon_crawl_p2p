use super::{FontAssets, InventoryTitle};
use crate::{
    player::{InventoryUpdatedEvent, LocalPlayer},
    prelude::*,
};
use bevy::render::view::RenderLayers;
use bevy_ggrs::LocalPlayers;

pub fn spawn_inventory_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn((
        InventoryTitle,
        Node {
            align_items: AlignItems::Center,
            display: Display::Flex,
            height: Val::Px(40.),
            margin: UiRect::all(Val::Px(10.)),
            overflow: Overflow::clip(),
            position_type: PositionType::Absolute,
            ..default()
        },
        RenderLayers::layer(config::HUD_CAMERA_RENDER_LAYER),
        Text::new("Inventory (0)"),
        TextColor(Color::WHITE),
        TextFont {
            font: font_assets.fira_sans_bold.clone(),
            font_size: 20.,
            ..default()
        },
    ));
}

pub fn update_inventory(
    mut events: EventReader<InventoryUpdatedEvent>,
    mut inventory_title: Query<&mut Text, With<InventoryTitle>>,
    local_players: Res<LocalPlayers>,
) {
    events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.player_id, &local_players))
        .map(|e| e.inventory.items.len())
        .for_each(|item_count| {
            inventory_title.single_mut().0 = format!("Inventory ({item_count})");
        });
}

use super::{FontAssets, InventoryItem, InventoryPanel, InventoryTitle};
use crate::{
    player::{InventoryUpdatedEvent, LocalPlayer},
    prelude::*,
};
use bevy::render::view::RenderLayers;
use bevy_ggrs::LocalPlayers;

pub fn spawn_inventory_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    let heading_font = TextFont {
        font: font_assets.fira_sans_bold.clone(),
        font_size: 20.,
        ..default()
    };

    commands
        .spawn((
            InventoryPanel,
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.5)),
            Node {
                align_items: AlignItems::Stretch,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                margin: UiRect::all(Val::Px(10.)),
                overflow: Overflow::scroll(),
                position_type: PositionType::Absolute,
                ..default()
            },
            RenderLayers::layer(config::HUD_CAMERA_RENDER_LAYER),
        ))
        .with_children(|parent| {
            parent.spawn((
                InventoryTitle,
                Text::new("Inventory (0)"),
                heading_font.clone(),
            ));
        });
}

pub fn update_inventory(
    mut commands: Commands,
    mut events: EventReader<InventoryUpdatedEvent>,
    mut inventory_title: Query<&mut Text, With<InventoryTitle>>,
    mut labels: Query<&mut Text, (With<InventoryItem>, Without<InventoryTitle>)>,
    font_assets: Res<FontAssets>,
    inventory_panel: Query<Entity, With<InventoryPanel>>,
    local_players: Res<LocalPlayers>,
) {
    let item_font = TextFont {
        font: font_assets.fira_sans_bold.clone(),
        font_size: 16.,
        ..default()
    };

    events
        .read()
        .filter(|e| LocalPlayer::is_local_player_id(e.player_id, &local_players))
        .for_each(|event| {
            inventory_title.single_mut().0 = format!("Inventory ({})", event.inventory.items.len());
            let panel = inventory_panel.single();

            // update UI items from revised inventory
            let mut item_count = 0;
            labels
                .iter_mut()
                .zip(event.inventory.items.iter().map(|item| item.label()))
                .for_each(|(mut label_entity, label)| {
                    item_count += 1;
                    label_entity.0 = format!("{item_count}: {label}");
                });

            // add any items missing from the UI
            event
                .inventory
                .items
                .iter()
                .skip(item_count)
                .for_each(|item| {
                    item_count += 1;
                    let item_entity = commands
                        .spawn((
                            InventoryItem,
                            Text(format!("{item_count}: {}", item.label())),
                            item_font.clone(),
                        ))
                        .id();
                    commands.entity(panel).add_child(item_entity);
                });

            // TODO remove extra UI items
        });
}

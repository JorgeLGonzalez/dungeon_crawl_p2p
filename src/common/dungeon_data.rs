use crate::items::MagicItemTemplate;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

#[derive(AssetCollection, Resource)]
pub struct DungeonAssets {
    #[asset(path = "dungeon.ron")]
    pub data: Handle<DungeonData>,
}

#[derive(Asset, Deserialize, TypePath)]
pub struct DungeonData {
    pub items: Vec<MagicItemTemplate>,
}

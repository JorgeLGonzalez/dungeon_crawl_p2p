use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

/**
 * REMEMBER to load assets in `App` in `main` via `add_loading_state`!
 */

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_bold: Handle<Font>,
}

use super::{
    ggrs::{checksum_transform, create_p2p_session, handle_ggrs_events},
    startup::startup,
};
use crate::{
    common::{DungeonAssets, DungeonData},
    hud,
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::GgrsApp;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<DungeonData>::new(&["ron"]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Startup)
                    .load_collection::<hud::FontAssets>()
                    .load_collection::<DungeonAssets>(),
            );

        app.add_systems(OnEnter(GameState::Startup), startup);

        if !game_mode(GameMode::SinglePlayer) {
            app.rollback_resource_with_clone::<RandomGenerator>()
                .checksum_resource_with_hash::<RandomGenerator>()
                .rollback_component_with_clone::<Transform>()
                .checksum_component::<Transform>(checksum_transform)
                .rollback_component_with_copy::<Visibility>();

            app.add_systems(
                Update,
                (
                    create_p2p_session.run_if(
                        in_state(GameState::Startup).and(|| game_mode(GameMode::MultiPlayer)),
                    ),
                    handle_ggrs_events.run_if(in_state(GameState::InGame)),
                ),
            );
        }
    }
}

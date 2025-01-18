use super::{
    assets::FontAssets,
    events::{DesyncEvent, SnapshotStateEvent},
    ggrs::{checksum_transform, create_p2p_session, handle_ggrs_events},
    random_generator::RandomGenerator,
    startup::startup,
};
use crate::config::GameMode;
use crate::{game_mode, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ggrs::GgrsApp;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Startup)
                .load_collection::<FontAssets>(),
        );

        app.add_event::<SnapshotStateEvent>()
            .add_systems(OnEnter(GameState::Startup), startup);

        if !game_mode(GameMode::SinglePlayer) {
            app.add_event::<DesyncEvent>();

            app.rollback_resource_with_clone::<RandomGenerator>()
            .rollback_component_with_clone::<Transform>()
                .checksum_resource_with_hash::<RandomGenerator>()
                .checksum_component::<Transform>(checksum_transform);

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

use super::{
    assets::FontAssets,
    events::DesyncEvent,
    ggrs::{checksum_transform, create_p2p_session, handle_ggrs_events},
    startup::startup,
};
use crate::resources::config::GameMode;
use crate::{game_mode, GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ggrs::GgrsApp;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Startup)
                .load_collection::<FontAssets>(),
        );

        app.add_event::<DesyncEvent>()
            .add_systems(OnEnter(GameState::Startup), startup);

        if !game_mode(GameMode::SinglePlayer) {
            app.checksum_component::<Transform>(checksum_transform);

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

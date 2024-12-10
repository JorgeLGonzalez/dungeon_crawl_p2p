use crate::resources::{config, MatchboxSocketResource};
use bevy::{log::info, prelude::Commands};
use bevy_matchbox::MatchboxSocket;

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = config::MATCHBOX_ROOM_URL;
    info!("Connecting to matchbox server {room_url}");
    commands.insert_resource(MatchboxSocketResource(MatchboxSocket::new_ggrs(room_url)));
}

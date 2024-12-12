use bevy_matchbox::prelude::PeerId;

pub const CAMERA_SCALE: f32 = 6.;
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 52;

pub const MAP_Z_LAYER: f32 = 10.;
pub const PLAYER_Z_LAYER: f32 = 100.;

pub const VIEWPORT_HEIGHT: f32 = 20.;

// RandomRoomsBuilder settings
pub const NUM_ROOMS: usize = 20;
pub const ROOM_MAX_WIDTH: usize = 10;
pub const ROOM_MAX_HEIGHT: usize = 10;

// Matchbox
pub const MATCHBOX_ROOM_URL: &str = "ws://127.0.0.1:3536/dungeon_crawl?next=2";
pub const NUM_PLAYERS: usize = 2;

pub type GgrsSessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

pub const PLAYER_SPEED: f32 = 10.;

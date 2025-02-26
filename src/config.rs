use crate::{
    fov::FovRadius,
    prelude::{TILE_HEIGHT, TILE_WIDTH},
};
use bevy::{color::Color, render::view::Layer};
use bevy_matchbox::prelude::PeerId;

// Game
#[cfg(not(target_arch = "wasm32"))]
pub const GAME_MODE: GameMode = GameMode::SinglePlayer;
#[cfg(target_arch = "wasm32")]
pub const GAME_MODE: GameMode = GameMode::MultiPlayer;

#[derive(Eq, PartialEq)]
pub enum GameMode {
    #[allow(dead_code)]
    GgrsSyncTest,
    MultiPlayer,
    SinglePlayer,
}

pub fn game_mode(mode: GameMode) -> bool {
    GAME_MODE == mode
}

// Camera
pub const CAMERA_RENDER_LAYER: Layer = 0;
pub const CAMERA_SCALE: f32 = 2.;

// Items
pub const ITEM_Z_LAYER: f32 = 80.;

// Monsters
pub const MONSTER_FOV_RADIUS: FovRadius = 6;
pub const MONSTER_Z_LAYER: f32 = 90.;
/// Probability that a monster will attempt to move in a given frame.
/// See monster_movement.
pub const MONSTER_MOVE_CHANCE: f64 = 0.01;
pub const MONSTER_THROTTLE_SECONDS: f32 = 0.15;
/// Auto save tracked monster moves when threshold is reached
pub const MONSTER_TRACKER_AUTO_SAVE_ENABLED: bool = false;
pub const MONSTER_TRACKER_AUTO_SAVE_THRESHOLD: usize = 100;

// Player
pub const PLAYER_0_COLOR: Color = Color::srgb(1., 1., 1.);
pub const PLAYER_1_COLOR: Color = Color::srgb(0., 1., 0.);
pub const PLAYER_WIDTH: f32 = TILE_WIDTH;
pub const PLAYER_HEIGHT: f32 = TILE_HEIGHT;
pub const PLAYER_FOV_RADIUS: FovRadius = 8;
/// Amount of seconds between auto-health increments
pub const PLAYER_HEALING_SECONDS: f32 = 10.0;
pub const PLAYER_HEALTH_MAX: u8 = 10;
/// Player is invincible for debugging purposes
pub const PLAYER_IMMORTAL: bool = true;
pub const PLAYER_MOVE_THROTTLE_SECONDS: f32 = 0.1;
pub const PLAYER_Z_LAYER: f32 = 100.;

// Matchbox and GGRS
pub const GGRS_DEBUG: bool = false;
pub const GGRS_INPUT_DELAY: usize = if GGRS_DEBUG { 0 } else { 2 };
// pub const MATCHBOX_ROOM_URL: &str = "ws://127.0.0.1:3536/dungeon_crawl?next=2";
pub const MATCHBOX_ROOM_URL: &str = "ws://3.147.199.67:3536/dungeon_crawl?next=2";
pub const NUM_PLAYERS: usize = 2;

pub type GgrsSessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

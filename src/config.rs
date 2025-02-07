use crate::fov::FovRadius;
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

// Dungeon view
pub const FLOOR_COLOR: Color = Color::srgb(0.3, 0.3, 0.3); // not illuminated: dark gray
pub const FLOOR_ILLUMINATED_COLOR: Color = Color::srgb(0.7, 0.7, 0.4); // illuminated: warm yellow-gray
pub const MAP_WIDTH: usize = 100;
pub const MAP_HEIGHT: usize = 52;
pub const MAP_Z_LAYER: f32 = 10.;
/// Min distance between starting position of player and any monster
pub const SAFETY_RADIUS: f32 = 10.;
pub const TILE_WIDTH: f32 = 1.;
pub const TILE_HEIGHT: f32 = TILE_WIDTH;
pub const VIEWPORT_HEIGHT: f32 = 20.;

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

// RandomRoomsBuilder settings
pub const NUM_ITEMS: usize = 30;
pub const NUM_MONSTERS: usize = 30;
pub const NUM_ROOMS: usize = 20;
pub const ROOM_MAX_WIDTH: usize = 10;
pub const ROOM_MAX_HEIGHT: usize = 10;

// Matchbox and GGRS
pub const GGRS_DEBUG: bool = false;
pub const GGRS_INPUT_DELAY: usize = if GGRS_DEBUG { 0 } else { 2 };
pub const MATCHBOX_ROOM_URL: &str = "ws://127.0.0.1:3536/dungeon_crawl?next=2";
pub const NUM_PLAYERS: usize = 2;

pub type GgrsSessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

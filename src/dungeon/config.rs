use bevy::color::Color;

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

pub const X_MAX: isize = (MAP_WIDTH / 2 - 1) as isize;
pub const X_MIN: isize = -((MAP_WIDTH / 2) as isize);
pub const Y_MAX: isize = ((MAP_HEIGHT / 2) - 1) as isize;
pub const Y_MIN: isize = -((MAP_HEIGHT / 2) as isize);

pub const VIEWPORT_HEIGHT: f32 = 20.;

// DrunkardsWalkBuilder settings
pub const PERCENT_FLOOR: usize = 33;
pub const STAGGER_DISTANCE: usize = 400;

// RandomRoomsBuilder settings
pub const NUM_ITEMS: usize = 30;
pub const NUM_MONSTERS: usize = 30;
pub const NUM_ROOMS: usize = 20;
pub const ROOM_MAX_WIDTH: usize = 10;
pub const ROOM_MAX_HEIGHT: usize = 10;

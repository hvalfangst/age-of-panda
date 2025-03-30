use std::time::Duration;

use crate::graphics::sprites::SpriteMaps;
use crate::state::player::Player;
use crate::Tile;
use minifb::Window;
use rodio::Source;

pub mod event_loop;
pub mod player;
pub mod core_logic;
pub mod physics;
mod gravity;
mod collision;

const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

const GRAVITY: f32 = 0.5;
pub const JUMP_VELOCITY: f32 = -5.0;
pub const MAX_VELOCITY: f32 = 2.0;
pub const ACCELERATION: f32 = 0.1;
const FRICTION: f32 = 0.2;
pub const GROUND: f32 = 205.0;
const LOWER_BOUND: f32 = 0.0;
const UPPER_BOUND: f32 = 225.0;
pub const KICK_FRAME_DURATION: u32 = 8;

pub const WALK_SOUND_1: usize = 0;
pub const WALK_SOUND_2: usize = 1;
pub const WALK_SOUND_3: usize = 2;
pub const WALK_SOUND_4: usize = 3;
pub const JUMP_SOUND: usize = 4;
const FALL_MILD_SOUND: usize = 5;
const FALL_HEAVY_SOUND: usize = 6;
const DOWN_SOUND: usize = 7;
const EXPLOSION_SOUND: usize = 8;
pub const KICK_SOUND: usize = 9;
pub const KICK_BOX_SOUND: usize = 10;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObstacleId(pub usize);

#[derive(Clone, Copy)]
pub struct Obstacle {
    pub id: ObstacleId,
    pub x_left: f32, // left x coordinate of the box (lower x value)
    pub x_right: f32, // right x coordinate of the box (higher x value)
    pub y_top: f32, // top y coordinate of the box (lower y value)
    pub y_bottom: f32, // bottom y coordinate of the box (higher y value)
    pub velocity_y: f32, // For gravity
    pub falling: bool,   // Whether it's falling
    pub active: bool,    // If false, box is removed
    pub durability: u8,  // Health of the box
    pub is_bottom_obstacle: bool, // Whether it's a bottom obstacle
    pub is_top_obstacle: bool,   // Whether it's a top obstacle
    pub is_leftmost_obstacle: bool, // Whether it's the leftmost obstacle
    pub is_rightmost_obstacle: bool, // Whether it's the rightmost obstacle
    pub left_obstacle: Option<ObstacleId>, // Id of the left obstacle
    pub right_obstacle: Option<ObstacleId>, // Id of the right obstacle
    pub over_obstacle: Option<ObstacleId>, // Id of the obstacle above
    pub under_obstacle: Option<ObstacleId> // Id of the obstacle below
}

pub struct Map<'a> {
    pub id: usize, // Unique identifier for the map
    pub tiles: Vec<Tile>, // Tiles for the map
    pub obstacles: &'a mut Vec<Obstacle>, // Obstacles for the map
    pub width: usize, // Width of the map
    pub height: usize, // Height of the map
    pub starting_x: f32, // Starting x coordinate for the player
    pub starting_y: f32, // Starting y coordinate for the player
    pub transition_x: f32, // Transition x coordinate for the player
    pub transition_y: f32  // Transition y coordinate for the player
}

pub struct GameState<'a> {
    pub player: Player, // Player object
    pub sprites: SpriteMaps, // Sprite maps
    pub window_buffer: &'a mut Vec<u32>, // Window buffer
    pub grass_sprite_index: usize, // Index of the grass sprite
    pub sky_sprite_index: usize, // Index of the sky sprite
    pub window_width: usize, // Width of the window
    pub window_height: usize, // Height of the window
    pub window: &'a mut Window, // Window object
    pub scaled_buffer: &'a mut Vec<u32>, // Scaled buffer
    pub game_over_index: usize, // Game over index
    pub all_maps: Vec<Map<'a>>, // All maps
    pub current_map_index: usize, // Current map index
    pub footstep_index: usize, // Footstep index
    pub footstep_active: bool, // Footstep active
    pub sounds: Vec<Vec<u8>>  // Sounds
}

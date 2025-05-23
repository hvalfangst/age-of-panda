use minifb::Key;

use crate::state::player::PlayerState::OnGround;
use crate::state::structs::Direction::Right;
use crate::state::structs::Direction;

// Define the states of the player
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    OnGround,
    AlmostGround,
    InAir,
    Walking,
    Attacking,
    Idle,
    OnObstacle
}
    pub struct Player {
        pub x: f32,
        pub y: f32,
        pub vx: f32,
        pub vy: f32,
        pub on_ground: bool,
        pub on_obstacle: bool,
        pub last_key: Option<Key>,
        pub left_increment: usize,
        pub right_increment: usize,
        pub direction: Direction,
        pub right_increment_frame_count: usize,
        pub left_increment_frame_count: usize,
        pub kick_start_time: u32,
        pub is_kicking: bool,
        pub kick_frame: usize,
        pub kick_frame_timer: usize,
        pub almost_ground: bool,
        pub is_jumping: bool,
        pub state: PlayerState,
        pub above_obstacle: bool,
        pub game_over: bool,
        pub obstacle_detected: bool,
        pub health: u8,
        pub invincible: bool
    }

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
            last_key: None,
            on_obstacle: false,
            left_increment: 4,
            right_increment: 0,
            direction: Right,
            right_increment_frame_count: 0,
            left_increment_frame_count: 0,
            is_kicking: false,
            kick_frame: 0,
            kick_frame_timer: 0,
            kick_start_time: 0,
            almost_ground: false,
            is_jumping: false,
            state: OnGround,
            above_obstacle: false,
            game_over: false,
            obstacle_detected: false,
            health: 3,
            invincible: false
        }
    }
}
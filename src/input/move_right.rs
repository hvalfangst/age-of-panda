use minifb::Key;
use rodio::Sink;
use crate::audio::engine::play_footstep_sound;
use crate::input::handler::InputLogic;
use crate::state::Direction::Right;
use crate::state::GameState;
use crate::state::physics::increase_velocity;

pub struct MoveRight;

impl InputLogic for MoveRight {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity
        increase_velocity(game_state);

        game_state.player.last_key = Some(Key::D);
        game_state.player.direction = Right;

        // Initialize a new field to track the frame count
        game_state.player.right_increment_frame_count += 1;

        if game_state.player.right_increment_frame_count >= 3 {
            game_state.player.right_increment_frame_count = 0; // Reset the frame count

            match game_state.player.right_increment {
                3 => {
                    game_state.player.right_increment = 0;
                }
                _ => {
                    game_state.player.right_increment += 1;
                }
            }
        }

        if game_state.footstep_active {
            play_footstep_sound(game_state, sink);
        }

    }
}
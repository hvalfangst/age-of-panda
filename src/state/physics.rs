use rodio::Sink;
use crate::audio::engine::append_source_source;
use crate::graphics::sprites::SpriteMaps;
use crate::state::{Direction, GameState, Obstacle, ACCELERATION, FALL_MILD_SOUND, GROUND, MAX_VELOCITY};
use crate::state::core_logic::CoreLogic;
use crate::state::player::{Player, PlayerState};

pub fn jump_obstacles(mut game_state: &mut GameState, sink: &mut rodio::Sink) {

    // Apply vertical velocity if jumping
    if game_state.player.is_jumping {
        game_state.player.y += game_state.player.vy;
    }

    // Check if game_state.player is almost on the ground
    if game_state.player.y >= 140.0 && game_state.player.y <= 160.0 {
        game_state.player.almost_ground = true;
    } else {
        game_state.player.almost_ground = false;
    }

    let mut on_any_obstacle = false;

    // Check for each obstacle
    for obstacle in game_state.all_maps[game_state.current_map_index].obstacles.iter() {

        if obstacle.active == false {
            continue;
        }

        if game_state.player.x + 10.0 > obstacle.x_left && game_state.player.x + 5.0 < obstacle.x_right {
            if game_state.player.y <= obstacle.y_bottom && game_state.player.y >= obstacle.y_top && obstacle.is_top_obstacle {
                // println!("game_state.player.y: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", game_state.player.y, obstacle.y_bottom, obstacle.y_top);
                if game_state.player.state != PlayerState::OnObstacle {
                    // player just landed on the obstacle
                    game_state.player.y = obstacle.y_bottom - 10.0;
                    game_state.player.on_obstacle = true;
                    game_state.player.on_ground = false;
                    game_state.player.is_jumping = false;
                    game_state.player.state = PlayerState::OnObstacle;
                    game_state.player.vy = 0.0;
                    // println!("Player is on an obstacle");
                } else {
                    // game_state.player is already on the obstacle
                    game_state.player.on_obstacle = true;
                    game_state.player.on_ground = false;
                }
                on_any_obstacle = true;
                break;
            } else if game_state.player.y < obstacle.y_top {
                // player is above the obstacle but not touching it
                game_state.player.on_ground = false;
                game_state.player.on_obstacle = false;
                game_state.player.above_obstacle = true;
                game_state.player.state = PlayerState::InAir;
                game_state.player.is_jumping = true;
                on_any_obstacle = true;
                break;
            }
        }
    }

    if !on_any_obstacle {
        if game_state.player.y >= GROUND {
            // player is on the ground (not on an obstacle)
            game_state.player.y = GROUND;
            game_state.player.vy = 0.0;
            game_state.player.on_ground = true;
            game_state.player.on_obstacle = false;
            game_state.player.is_jumping = false;

            if game_state.player.state == PlayerState::InAir {
                append_source_source(&game_state, sink, FALL_MILD_SOUND, 2500);
            }

            game_state.player.state = PlayerState::OnGround;

            // println!("Player is on the ground");
        } else {
            // player is in the air (not above any obstacle)
            game_state.player.on_ground = false;
            game_state.player.on_obstacle = false;
            game_state.player.above_obstacle = false;
            game_state.player.state = PlayerState::InAir;
            game_state.player.is_jumping = true;
            // println!("Player is in the air");
        }
    }
}

pub fn increase_velocity(game_state: &mut GameState) {
    game_state.player.vx += ACCELERATION;

    if game_state.player.obstacle_detected {
        game_state.player.vx = 0.0;
    } else {
        if game_state.player.vx > MAX_VELOCITY {
            game_state.player.vx = MAX_VELOCITY;
        } else {
            game_state.player.vx *= 0.98;
            if game_state.player.vx > MAX_VELOCITY {
                game_state.player.vx = MAX_VELOCITY;
            }
        }
    }
}

pub fn decrease_velocity(game_state: &mut GameState) {
    game_state.player.vx *= 0.95;
    if game_state.player.vx.abs() < 0.1 {
        game_state.player.vx = 0.0;
    }
}

pub fn check_collision(obstacles: &Vec<Obstacle>, sprites: &SpriteMaps, player: &Player, is_left: bool) -> (bool, Option<usize>) {
    let mut collision_id: Option<usize> = None;
    // println!("----------------------------------------------------------------------");
    let collision = obstacles.iter().enumerate().any(|(index, obstacle)| {
        // println!("Checking collision: id: {:?}, x_left: {}, x_right: {}, y_bottom: {}, y_top: {}", obstacle.id, obstacle.x_left, obstacle.x_right, obstacle.y_bottom, obstacle.y_top);

        if obstacle.active == false {
            // println!("- - - - Obstacle is not active - - - -");
            return false;
        }

        let player_x = if is_left {
            player.x + (sprites.player[player.left_increment].width as f32 / 2.5)
        } else {
            player.x + (sprites.player[player.right_increment].width as f32 / 1.5)
        };

        if player_x > obstacle.x_left && player_x < obstacle.x_right {
            // println!("Player y: {}, obs.y_bottom: {}, obs.y_top: {}", player.y, obstacle.y_bottom, obstacle.y_top);

            if player.y >= obstacle.y_top + 10.0 && player.y <= obstacle.y_bottom + 25.0 {
                // println!("Collision of x detected: p_x: {}, obs.x_left: {}, obs.x_right: {}", player_x, obstacle.x_left, obstacle.x_right);

                collision_id = Some(index);
                // println!("Collision detected with obstacle id {:?} x.left {}, x.right: {}, obstacle.y_bottom: {}, obstacle.y_top: {}", obstacle.id, obstacle.x_left, obstacle.x_right , obstacle.y_bottom + 25.0, obstacle.y_top + 25.0);
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    if let Some(id) = collision_id {
    }

    (collision, collision_id)
}

pub struct CheckTrapCollision;

impl CoreLogic for CheckTrapCollision {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let traps = &game_state.all_maps[game_state.current_map_index].traps;
        let player = &game_state.player;

        for trap in traps.iter() {
            if trap.active == false {
                continue;
            }
            if player.x + 10.0 > trap.x_left && player.x + 5.0 < trap.x_right {
                if player.y <= trap.y_bottom && player.y >= trap.y_top {
                    game_state.player.health -= 1;
                    game_state.player.x = trap.x_left - 64.0;
                    game_state.player.vy = 2.0;

                    game_state.layer_0_index = 1;

                    if game_state.player.health == 0 {
                        game_state.player.game_over = true;
                    }

                    break;
                }
            }
        }
    }
}

pub struct ModifyPosition;

impl CoreLogic for ModifyPosition {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.direction == Direction::Left {
            game_state.player.x -= game_state.player.vx;
        } else {
            game_state.player.x += game_state.player.vx;
        }
        game_state.player.y += game_state.player.vy;
    }
}
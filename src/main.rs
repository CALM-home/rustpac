use macroquad::prelude::*;

mod player;
mod platform;
mod rocket;
mod fuel;

use player::Player;
use platform::Platform;
use rocket::{Rocket, ModuleState};
use fuel::{FuelSystem, MAX_CAPSULES};

/// États du jeu
#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Playing,
    Launching,       // Animation de décollage
    LevelTransition, // Écran entre les niveaux
}

struct Game {
    state: GameState,
    level: u32,
    score: u32,
    launch_timer: f32,
    launch_speed: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Playing,
            level: 1,
            score: 0,
            launch_timer: 0.0,
            launch_speed: 0.0,
        }
    }

    fn start_launch(&mut self) {
        self.state = GameState::Launching;
        self.launch_timer = 0.0;
        self.launch_speed = 0.5;  // Plus lent au démarrage
    }

    fn next_level(&mut self) {
        self.level += 1;
        self.state = GameState::Playing;
        self.launch_timer = 0.0;
        self.launch_speed = 0.0;
    }

    fn reset(&mut self) {
        self.state = GameState::Playing;
        self.level = 1;
        self.score = 0;
        self.launch_timer = 0.0;
        self.launch_speed = 0.0;
    }
}

#[macroquad::main("RustPac - Jetpac Clone")]
async fn main() {
    // Initialize game
    let mut game = Game::new();
    
    // Initialize player
    let mut player = Player::new(screen_width() / 2.0 - 12.0, 50.0);

    // Initialize rocket
    let mut rocket = Rocket::new(0.0, 0.0);
    let mut modules_spawned = false;

    // Track carried module
    let mut carried_module_index: Option<usize> = None;

    // Initialize fuel system
    let mut fuel_system: Option<FuelSystem> = None;
    
    println!("RustPac - Step 5: Launch and Levels!");
    println!("Controls: ← → to move, ↑ or SPACE to thrust");
    println!("1. Assemble rocket: bottom → middle → top");
    println!("2. Collect fuel capsules and drop them on the rocket");
    println!("3. Board the rocket when full to launch!");

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Update platform
        let platform = Platform::ground_platform(screen_w, screen_h);

        // Spawn modules on first frame or level reset
        if !modules_spawned {
            rocket.base_position = vec2(
                (screen_w - rocket::MODULE_SIZE) / 2.0,
                platform.rect.y - rocket::MODULE_SIZE,
            );
            rocket.spawn_modules(screen_w, screen_h);
            modules_spawned = true;
            // Reset player position for new level
            player = Player::new(screen_w / 2.0 - 12.0, 50.0);
        }

        // ===== GAME STATE: PLAYING =====
        if game.state == GameState::Playing {
            // Check landing on platform
            if let Some(land_y) = platform.check_landing(player.pos, player.size, player.vel.y) {
                player.land(land_y);
            }

            // Update player
            player.update();

            // Auto-pickup: check if we can grab a module by flying over it (only if nothing falling)
            if carried_module_index.is_none() && !rocket.has_falling_module() {
                if let Some(next_type) = rocket.get_next_module_type() {
                    for (i, module) in rocket.modules.iter().enumerate() {
                        if module.check_pickup(player.pos, player.size, next_type) {
                            rocket.modules[i].set_carried();
                            carried_module_index = Some(i);
                            break;
                        }
                    }
                }
            }

            // Auto-drop: start falling when player is in assembly column
            if let Some(index) = carried_module_index {
                if rocket.is_in_assembly_column(player.pos, player.size) && !rocket.has_falling_module() {
                    let carried_type = rocket.modules[index].module_type;
                    if rocket.can_place_type(carried_type) {
                        let target_pos = rocket.get_placement_pos_for_type(carried_type);
                        let start_pos = vec2(player.pos.x, player.pos.y - rocket::MODULE_SIZE - 5.0);
                        rocket.modules[index].start_falling(start_pos, target_pos);
                        carried_module_index = None;
                    }
                }
            }

            // Update falling modules physics
            rocket.update_falling();

            // Initialize fuel system when rocket is complete
            if rocket.is_complete() && fuel_system.is_none() {
                let tank_pos = vec2(
                    rocket.base_position.x + rocket::MODULE_SIZE / 2.0 - fuel::FUEL_SIZE / 2.0,
                    rocket.base_position.y - rocket::MODULE_SIZE - fuel::FUEL_SIZE / 2.0,
                );
                fuel_system = Some(FuelSystem::new(tank_pos));
            }

            // Update fuel system
            if let Some(ref mut fuel) = fuel_system {
                fuel.update_spawning();
                fuel.update_falling();

                // Auto-pickup fuel
                if carried_module_index.is_none() && fuel.find_carried().is_none() && !fuel.has_falling() && !fuel.is_full() {
                    for (i, capsule) in fuel.capsules.iter().enumerate() {
                        if capsule.check_pickup(player.pos, player.size) {
                            fuel.capsules[i].set_carried();
                            break;
                        }
                    }
                }

                // Auto-drop fuel
                if let Some(index) = fuel.find_carried() {
                    if rocket.is_in_assembly_column(player.pos, player.size) && !fuel.has_falling() {
                        let target_pos = vec2(
                            fuel.fuel_tank_pos.x - fuel::FUEL_SIZE / 2.0,
                            fuel.fuel_tank_pos.y + fuel::FUEL_SIZE / 2.0,
                        );
                        let start_pos = vec2(player.pos.x + 2.0, player.pos.y - fuel::FUEL_SIZE - 8.0);
                        fuel.capsules[index].start_falling(start_pos, target_pos);
                    }
                }

                // CHECK LAUNCH CONDITION: Fuel full + player on rocket
                if fuel.is_full() && game.state == GameState::Playing {
                    // Check if player is standing on top of the rocket
                    let rocket_top = rocket.base_position.y - rocket::MODULE_SIZE * 3.0;
                    let rocket_left = rocket.base_position.x;
                    let rocket_right = rocket.base_position.x + rocket::MODULE_SIZE;
                    
                    let player_center_x = player.pos.x + player.size / 2.0;
                    let player_bottom = player.pos.y + player.size;
                    
                    let on_rocket_x = player_center_x >= rocket_left - 5.0 && player_center_x <= rocket_right + 5.0;
                    let on_rocket_y = player_bottom >= rocket_top - 10.0 && player_bottom <= rocket_top + 15.0;
                    
                    if on_rocket_x && on_rocket_y && player.vel.y >= 0.0 {
                        // Player is on the rocket! Start launch sequence
                        game.start_launch();
                    }
                }
            }
        }
        
        // ===== GAME STATE: LAUNCHING =====
        else if game.state == GameState::Launching {
            game.launch_timer += get_frame_time();
            
            // Accelerate rocket upward (plus lent)
            game.launch_speed += 0.03;
            
            // Move rocket base up
            rocket.base_position.y -= game.launch_speed;
            
            // Move all placed modules up with the rocket
            for module in rocket.modules.iter_mut() {
                if let rocket::ModuleState::Placed { ref mut pos } = module.state {
                    pos.y -= game.launch_speed;
                }
            }
            
            // After rocket leaves screen, go to level transition
            if rocket.base_position.y < -100.0 || game.launch_timer > 6.0 {
                game.state = GameState::LevelTransition;
            }
        }
        
        // ===== GAME STATE: LEVEL TRANSITION =====
        else if game.state == GameState::LevelTransition {
            // Wait for keypress to continue
            if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                // Add score for completing level
                game.score += 100 * game.level;
                game.next_level();
                // Reset level state
                modules_spawned = false;
                carried_module_index = None;
                fuel_system = None;
                rocket = Rocket::new(0.0, 0.0);
            }
        }

        // ===== DRAW: PLAYING or LAUNCHING =====
        if game.state == GameState::Playing || game.state == GameState::Launching {
            clear_background(BLACK);
            
            platform.draw();
            
            // Draw rocket (with carried module following player)
            let player_pos_for_carry = carried_module_index.map(|_| player.pos);
            rocket.draw(player_pos_for_carry);

            // Draw fuel system
            if let Some(ref fuel) = fuel_system {
                let fuel_carried_pos = fuel.find_carried().map(|_| player.pos);
                fuel.draw(fuel_carried_pos);
            }

            // Player is hidden during launch (inside the rocket!)
            if game.state != GameState::Launching {
                player.draw();
            }
            
            // Launch effect: flames from rocket
            if game.state == GameState::Launching {
                let flame_x = rocket.base_position.x + rocket::MODULE_SIZE / 2.0;
                let flame_y = rocket.base_position.y + rocket::MODULE_SIZE;
                let flame_height = 30.0 + rand::gen_range(0.0, 20.0);
                draw_rectangle(flame_x - 6.0, flame_y, 12.0, flame_height, ORANGE);
                draw_rectangle(flame_x - 3.0, flame_y, 6.0, flame_height + 10.0, YELLOW);
            }
        }
        
        // ===== UI: PLAYING =====
        if game.state == GameState::Playing {
            // Level and score
            draw_text(&format!("RustPac - Level {}", game.level), 10.0, 30.0, 24.0, WHITE);
            draw_text(&format!("Score: {}", game.score), screen_w - 150.0, 30.0, 20.0, YELLOW);
            draw_text("← → : move | ↑ : thrust | ESC : quit", 10.0, 55.0, 16.0, GRAY);

            // Instructions based on state
            if !rocket.is_complete() {
                draw_text("Fly OVER modules to pick them up (in order!)", 10.0, 75.0, 14.0, YELLOW);
            } else if let Some(ref fuel) = fuel_system {
                if fuel.is_full() {
                    draw_text("LAND ON THE ROCKET TO LAUNCH! 🚀", 10.0, 75.0, 18.0, GREEN);
                } else if fuel.all_delivered() {
                    draw_text("ALL FUEL DELIVERED! Ready for launch! 🚀", 10.0, 75.0, 16.0, GREEN);
                } else {
                    draw_text("Fly OVER fuel capsule, DROP it on rocket, then next appears", 10.0, 75.0, 14.0, ORANGE);
                }
            }

            // Status bar
            let status = if let Some(ref fuel) = fuel_system {
                if fuel.is_full() {
                    "LAND ON ROCKET TO LAUNCH!"
                } else if fuel.all_delivered() {
                    "FUEL DEPOSITED - WAIT..."
                } else if fuel.has_falling() {
                    "FUEL DELIVERING..."
                } else if fuel.find_carried().is_some() {
                    "CARRYING FUEL → Drop on rocket"
                } else {
                    "Find the fuel capsule!"
                }
            } else if rocket.is_complete() {
                "ROCKET COMPLETE! 🚀"
            } else if rocket.has_falling_module() {
                "MODULE FALLING..."
            } else if carried_module_index.is_some() {
                "CARRYING MODULE → Fly over rocket"
            } else {
                "Fly over the next module"
            };
            let status_color = if rocket.has_falling_module() || fuel_system.as_ref().map(|f| f.has_falling()).unwrap_or(false) {
                YELLOW
            } else if fuel_system.as_ref().map(|f| f.is_full()).unwrap_or(false) {
                GREEN
            } else {
                WHITE
            };
            draw_text(status, 10.0, screen_h - 40.0, 16.0, status_color);

            // Progress
            if !rocket.is_complete() {
                let placed_count = rocket.modules.iter().filter(|m| matches!(m.state, ModuleState::Placed { .. })).count();
                draw_text(&format!("Modules: {}/{}", placed_count, 3), screen_w - 120.0, screen_h - 20.0, 16.0, YELLOW);
            }

            // Fuel gauge
            if let Some(ref fuel) = fuel_system {
                fuel.draw_gauge(screen_w - 150.0, screen_h - 25.0, 140.0, 20.0);
                let remaining_text = format!("Capsules: {}/{} done", fuel.capsules_delivered, MAX_CAPSULES);
                draw_text(&remaining_text, screen_w - 150.0, screen_h - 45.0, 14.0, GRAY);
            }
        }
        
        // ===== UI: LAUNCHING =====
        else if game.state == GameState::Launching {
            draw_text("LIFTOFF! 🚀", screen_w / 2.0 - 80.0, screen_h / 2.0, 36.0, YELLOW);
        }
        
        // ===== UI: LEVEL TRANSITION =====
        else if game.state == GameState::LevelTransition {
            clear_background(BLACK);
            draw_text("LEVEL COMPLETE!", screen_w / 2.0 - 120.0, screen_h / 3.0, 36.0, GREEN);
            draw_text(&format!("Level {} cleared!", game.level), screen_w / 2.0 - 80.0, screen_h / 2.0 - 20.0, 24.0, WHITE);
            draw_text(&format!("Score: {}", game.score), screen_w / 2.0 - 60.0, screen_h / 2.0 + 20.0, 20.0, YELLOW);
            draw_text("Press SPACE to continue to next level", screen_w / 2.0 - 180.0, screen_h * 2.0 / 3.0, 20.0, SKYBLUE);
        }

        // Exit
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

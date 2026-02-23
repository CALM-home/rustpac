use macroquad::prelude::*;

mod player;
mod platform;
mod rocket;
mod fuel;

use player::Player;
use platform::Platform;
use rocket::{Rocket, ModuleState};
use fuel::{FuelSystem, MAX_CAPSULES};

#[macroquad::main("RustPac - Jetpac Clone")]
async fn main() {
    // Initialize player
    let mut player = Player::new(screen_width() / 2.0 - 12.0, 50.0);

    // Initialize rocket
    let mut rocket = Rocket::new(0.0, 0.0);
    let mut modules_spawned = false;

    // Track carried module
    let mut carried_module_index: Option<usize> = None;

    // Initialize fuel system
    let mut fuel_system: Option<FuelSystem> = None;
    
    println!("RustPac - Step 4: Fuel the Rocket");
    println!("Controls: ← → to move, ↑ or SPACE to thrust");
    println!("1. Assemble rocket: bottom → middle → top");
    println!("2. Collect fuel capsules and drop them on the rocket");

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Update platform
        let platform = Platform::ground_platform(screen_w, screen_h);

        // Spawn modules on first frame
        if !modules_spawned {
            rocket.base_position = vec2(
                (screen_w - rocket::MODULE_SIZE) / 2.0,
                platform.rect.y - rocket::MODULE_SIZE,
            );
            rocket.spawn_modules(screen_w, screen_h);
            modules_spawned = true;
        }

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
            // Fuel tank is at the middle section of the rocket
            let tank_pos = vec2(
                rocket.base_position.x + rocket::MODULE_SIZE / 2.0 - fuel::FUEL_SIZE / 2.0,
                rocket.base_position.y - rocket::MODULE_SIZE - fuel::FUEL_SIZE / 2.0,
            );
            fuel_system = Some(FuelSystem::new(tank_pos));
        }

        // Update fuel system
        if let Some(ref mut fuel) = fuel_system {
            // Spawn next capsule if previous was delivered
            fuel.update_spawning();

            // Update falling capsules
            fuel.update_falling();

            // Auto-pickup fuel (only if not carrying module and not carrying fuel and not falling)
            if carried_module_index.is_none() && fuel.find_carried().is_none() && !fuel.has_falling() && !fuel.is_full() {
                for (i, capsule) in fuel.capsules.iter().enumerate() {
                    if capsule.check_pickup(player.pos, player.size) {
                        fuel.capsules[i].set_carried();
                        break;
                    }
                }
            }

            // Auto-drop fuel when in rocket column and rocket is complete
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
        }

        // Draw everything
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

        player.draw();
        
        // UI
        let step_text = if rocket.is_complete() {
            "RustPac - Step 4: Fuel the Rocket"
        } else {
            "RustPac - Step 3: Rocket Assembly"
        };
        draw_text(step_text, 10.0, 30.0, 24.0, WHITE);
        draw_text("← → : move | ↑ : thrust | ESC : quit", 10.0, 55.0, 16.0, GRAY);

        // Instructions based on state
        if !rocket.is_complete() {
            draw_text("Fly OVER modules to pick them up (in order!)", 10.0, 75.0, 14.0, YELLOW);
        } else if let Some(ref fuel) = fuel_system {
            if fuel.all_delivered() {
                draw_text("ALL FUEL DELIVERED! Ready for launch! 🚀", 10.0, 75.0, 16.0, GREEN);
            } else {
                draw_text("Fly OVER fuel capsule, DROP it on rocket, then next appears", 10.0, 75.0, 14.0, ORANGE);
            }
        }

        // Status
        let status = if let Some(ref fuel) = fuel_system {
            if fuel.all_delivered() {
                "READY FOR LAUNCH! 🚀"
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
        } else if fuel_system.as_ref().map(|f| f.all_delivered()).unwrap_or(false) {
            GREEN
        } else {
            WHITE
        };
        draw_text(status, 10.0, screen_h - 40.0, 16.0, status_color);

        // Progress
        if !rocket.is_complete() {
            let placed_count = rocket.modules.iter().filter(|m| matches!(m.state, ModuleState::Placed { .. })).count();
            draw_text(&format!("Modules: {}/3", placed_count), screen_w - 120.0, screen_h - 20.0, 16.0, YELLOW);
        }

        // Fuel gauge and remaining capsules
        if let Some(ref fuel) = fuel_system {
            fuel.draw_gauge(screen_w - 150.0, screen_h - 25.0, 140.0, 20.0);
            // Show remaining capsules
            let remaining = fuel.capsules_remaining();
            let remaining_text = format!("Capsules: {}/{} done", fuel.capsules_delivered, MAX_CAPSULES);
            draw_text(&remaining_text, screen_w - 150.0, screen_h - 45.0, 14.0, GRAY);
        }

        // Exit
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

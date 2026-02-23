use macroquad::prelude::*;

mod player;
mod platform;
mod rocket;

use player::Player;
use platform::Platform;
use rocket::{Rocket, ModuleState};

#[macroquad::main("RustPac - Jetpac Clone")]
async fn main() {
    // Initialize player
    let mut player = Player::new(screen_width() / 2.0 - 12.0, 50.0);
    
    // Initialize rocket
    let mut rocket = Rocket::new(0.0, 0.0);
    let mut modules_spawned = false;
    
    // Track carried module
    let mut carried_module_index: Option<usize> = None;
    
    println!("RustPac - Controls: ← → to move, ↑ or SPACE to thrust");
    println!("Fly over modules to pick them up (in order: bottom → middle → top)");

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
                if let Some(next_type) = rocket.get_next_placement_type() {
                    let target_pos = rocket.get_placement_pos(next_type);
                    let start_pos = vec2(player.pos.x, player.pos.y - rocket::MODULE_SIZE - 5.0);
                    rocket.modules[index].start_falling(start_pos, target_pos);
                    carried_module_index = None;
                }
            }
        }

        // Update falling modules physics
        rocket.update_falling();

        // Draw everything
        clear_background(BLACK);
        
        platform.draw();
        
        // Draw rocket (with carried module following player)
        let player_pos_for_carry = carried_module_index.map(|_| player.pos);
        rocket.draw(player_pos_for_carry);
        
        player.draw();
        
        // UI
        draw_text("RustPac - Step 3: Rocket Assembly", 10.0, 30.0, 24.0, WHITE);
        draw_text("← → : move | ↑ : thrust | ESC : quit", 10.0, 55.0, 16.0, GRAY);
        draw_text("Fly OVER modules to pick them up (in order!)", 10.0, 75.0, 14.0, YELLOW);
        
        // Status
        let status = if rocket.is_complete() {
            "ROCKET COMPLETE! 🚀"
        } else if rocket.has_falling_module() {
            "MODULE FALLING... Watch it land!"
        } else if carried_module_index.is_some() {
            "CARRYING → Fly over rocket column to drop"
        } else {
            "Fly over the next module to pick it up"
        };
        let status_color = if rocket.has_falling_module() { YELLOW } else { GREEN };
        draw_text(status, 10.0, screen_h - 20.0, 16.0, status_color);

        // Progress
        let placed_count = rocket.modules.iter().filter(|m| matches!(m.state, ModuleState::Placed { .. })).count();
        draw_text(&format!("Progress: {}/3", placed_count), screen_w - 100.0, screen_h - 20.0, 16.0, YELLOW);

        // Exit
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

use macroquad::prelude::*;

mod player;
mod platform;

use player::Player;
use platform::Platform;

#[macroquad::main("RustPac - Jetpac Clone")]
async fn main() {
    // Initialize player above the platform
    let mut player = Player::new(screen_width() / 2.0 - 12.0, 50.0);
    
    println!("RustPac - Controls: ← → to move, ↑ or SPACE to thrust");

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        // Create platform (recreate each frame to handle window resize)
        let platform = Platform::ground_platform(screen_w, screen_h);

        // Check landing on platform
        if let Some(land_y) = platform.check_landing(player.pos, player.size, player.vel.y) {
            player.land(land_y);
        }

        // Update player
        player.update();

        // Draw everything
        clear_background(BLACK);
        
        platform.draw();
        player.draw();
        
        // UI
        draw_text("RustPac - Step 2: Landing Pad", 10.0, 30.0, 24.0, WHITE);
        draw_text("← → : move | ↑ or SPACE : thrust | ESC : quit", 10.0, 55.0, 16.0, GRAY);
        
        // Debug info
        let ground_status = if player.on_ground { "LANDED" } else { "FLYING" };
        draw_text(&format!("Status: {}", ground_status), 10.0, screen_h - 20.0, 16.0, GREEN);

        // Exit
        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

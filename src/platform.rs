use macroquad::prelude::*;

const PLATFORM_COLOR: Color = GRAY;

pub struct Platform {
    pub rect: Rect,
    pub color: Color,
}

impl Platform {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            color: PLATFORM_COLOR,
        }
    }

    /// Platform at the bottom spanning full screen width (like original Jetpac)
    pub fn ground_platform(screen_width: f32, screen_height: f32) -> Self {
        let width = screen_width;
        let height = 20.0;
        let x = 0.0;
        let y = screen_height - height;
        Self::new(x, y, width, height)
    }

    pub fn draw(&self) {
        // Main platform body
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);
        
        // Top highlight line for depth
        draw_line(
            self.rect.x,
            self.rect.y,
            self.rect.x + self.rect.w,
            self.rect.y,
            2.0,
            LIGHTGRAY,
        );
    }

    /// Check if a player (at position with size) is standing on this platform
    pub fn check_landing(&self, player_pos: Vec2, player_size: f32, player_vel_y: f32) -> Option<f32> {
        // Player feet position
        let player_feet_y = player_pos.y + player_size;
        let player_center_x = player_pos.x + player_size / 2.0;

        // Check horizontal overlap
        if player_center_x >= self.rect.x && player_center_x <= self.rect.x + self.rect.w {
            // Check if feet are near the platform top (with small tolerance)
            let tolerance = 5.0;
            if player_feet_y >= self.rect.y - tolerance 
                && player_feet_y <= self.rect.y + self.rect.h / 2.0
                && player_vel_y >= 0.0 { // Only land when falling or stationary
                return Some(self.rect.y - player_size);
            }
        }
        None
    }
}

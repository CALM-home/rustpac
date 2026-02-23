use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 24.0;
const PLAYER_COLOR: Color = Color::new(0.2, 0.8, 0.3, 1.0); // Vert
const GRAVITY: f32 = 0.10;        // Gravité très faible
const THRUST: f32 = 0.22;         // Poussée plus douce
const MOVE_ACCEL: f32 = 0.08;     // Accélération très progressive
const MAX_SPEED: f32 = 2.0;       // Vitesse max encore réduite
const FRICTION: f32 = 0.992;      // Presque pas de frottement = glisse beaucoup

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub size: f32,
    pub on_ground: bool,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(0.0, 0.0),
            size: PLAYER_SIZE,
            on_ground: false,
        }
    }

    /// Land on a platform at specific Y position
    pub fn land(&mut self, y_position: f32) {
        self.pos.y = y_position;
        self.vel.y = 0.0;
        self.on_ground = true;
    }

    /// Reset ground state (call before collision checks each frame)
    pub fn reset_ground_state(&mut self) {
        self.on_ground = false;
    }

    pub fn update(&mut self) {
        // Reset ground state - will be set by collision if needed
        self.reset_ground_state();

        // Gravité (only if not on ground)
        if !self.on_ground {
            self.vel.y += GRAVITY;
        }

        // Contrôles - accélération progressive pour plus d'inertie
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.vel.x -= MOVE_ACCEL;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.vel.x += MOVE_ACCEL;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Space) {
            self.vel.y -= THRUST;
        }

        // Frottement très faible pour garder l'inertie (style Jetpac)
        self.vel.x *= FRICTION;

        // Limitation de vitesse
        self.vel.x = self.vel.x.clamp(-MAX_SPEED, MAX_SPEED);
        self.vel.y = self.vel.y.clamp(-MAX_SPEED, MAX_SPEED * 1.2);

        // Mise à jour position
        self.pos += self.vel;

        // Limites d'écran
        let screen_w = screen_width();
        let screen_h = screen_height();
        
        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.vel.x = 0.0;
        }
        if self.pos.x + self.size > screen_w {
            self.pos.x = screen_w - self.size;
            self.vel.x = 0.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.vel.y = 0.0;
        }
        // Bottom limit removed - platforms handle landing now
    }

    pub fn draw(&self) {
        // Corps du joueur (carré vert)
        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, PLAYER_COLOR);
        
        // Petit détail : visière
        draw_rectangle(
            self.pos.x + self.size * 0.6,
            self.pos.y + self.size * 0.2,
            self.size * 0.3,
            self.size * 0.3,
            DARKBLUE,
        );

        // Effet de propulsion quand on monte
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) || is_key_down(KeyCode::Space) {
            let flame_height = rand::gen_range(8.0, 16.0);
            draw_rectangle(
                self.pos.x + self.size * 0.3,
                self.pos.y + self.size,
                self.size * 0.4,
                flame_height,
                ORANGE,
            );
        }
    }

    pub fn center(&self) -> Vec2 {
        vec2(
            self.pos.x + self.size / 2.0,
            self.pos.y + self.size / 2.0,
        )
    }
}

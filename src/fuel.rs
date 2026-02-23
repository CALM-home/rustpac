use macroquad::prelude::*;
use macroquad::window::{screen_width, screen_height};

pub const FUEL_SIZE: f32 = 20.0;
const FUEL_COLOR: Color = Color::new(0.9, 0.3, 0.1, 1.0); // Rouge-orange vif
const FUEL_GLOW: Color = Color::new(1.0, 0.5, 0.2, 0.6);   // Aura lumineuse

const FUEL_GRAVITY: f32 = 0.15;
const FUEL_MAX_FALL_SPEED: f32 = 3.0;
const FUEL_HORIZONTAL_SPEED: f32 = 0.08;

/// Quantité de carburant par capsule (3 capsules = 100%)
pub const FUEL_PER_CAPSULE: f32 = 100.0 / 3.0;
/// Capacité maximale de la fusée
pub const FUEL_CAPACITY: f32 = 100.0;

#[derive(Clone, Copy, PartialEq)]
pub enum FuelState {
    Spawned { pos: Vec2 },      // Au sol, à ramasser
    Carried,                     // Transporté par le joueur
    Falling { pos: Vec2, vel_y: f32, target_pos: Vec2 }, // En chute vers réservoir
}

pub struct FuelCapsule {
    pub state: FuelState,
    pub amount: f32,
    pub size: f32,
}

impl FuelCapsule {
    pub fn new(pos: Vec2) -> Self {
        Self {
            state: FuelState::Spawned { pos },
            amount: FUEL_PER_CAPSULE,
            size: FUEL_SIZE,
        }
    }

    pub fn set_carried(&mut self) {
        self.state = FuelState::Carried;
    }

    pub fn start_falling(&mut self, start_pos: Vec2, target_pos: Vec2) {
        self.state = FuelState::Falling {
            pos: start_pos,
            vel_y: 0.0,
            target_pos,
        };
    }

    /// Update falling physics, returns true if landed
    pub fn update_falling(&mut self) -> bool {
        if let FuelState::Falling { pos, vel_y, target_pos } = &mut self.state {
            // Apply gravity
            *vel_y += FUEL_GRAVITY;
            if *vel_y > FUEL_MAX_FALL_SPEED {
                *vel_y = FUEL_MAX_FALL_SPEED;
            }

            // Move down
            pos.y += *vel_y;

            // Horizontal alignment
            let x_diff = target_pos.x - pos.x;
            pos.x += x_diff * FUEL_HORIZONTAL_SPEED;

            // Check if reached target
            if pos.y >= target_pos.y && x_diff.abs() < 1.0 {
                *pos = *target_pos;
                return true;
            }
        }
        false
    }

    /// Check if player can pick up this fuel capsule
    pub fn check_pickup(&self, player_pos: Vec2, player_size: f32) -> bool {
        // Can't pick up if falling or already deposited
        if matches!(self.state, FuelState::Falling { .. }) {
            return false;
        }

        if let FuelState::Spawned { pos } = self.state {
            let player_center_x = player_pos.x + player_size / 2.0;
            let capsule_center_x = pos.x + self.size / 2.0;
            let player_bottom = player_pos.y + player_size;
            let capsule_top = pos.y;

            // Vertical alignment
            let x_aligned = (player_center_x - capsule_center_x).abs() < self.size * 0.6;
            // Vertical proximity
            let y_close = player_bottom >= capsule_top - 5.0 && player_bottom <= capsule_top + self.size;

            return x_aligned && y_close;
        }
        false
    }

    pub fn draw(&self, player_pos: Option<Vec2>) {
        let pos = match self.state {
            FuelState::Spawned { pos } => pos,
            FuelState::Falling { pos, .. } => pos,
            FuelState::Carried => {
                if let Some(p_pos) = player_pos {
                    vec2(p_pos.x + 2.0, p_pos.y - self.size - 8.0)
                } else {
                    return;
                }
            }
        };

        // Glow effect
        draw_circle(pos.x + self.size / 2.0, pos.y + self.size / 2.0, self.size * 0.8, FUEL_GLOW);

        // Main capsule body (rounded rectangle-ish)
        draw_rectangle(pos.x + 2.0, pos.y, self.size - 4.0, self.size, FUEL_COLOR);
        draw_circle(pos.x + self.size / 2.0, pos.y + 2.0, (self.size - 4.0) / 2.0, FUEL_COLOR);
        draw_circle(pos.x + self.size / 2.0, pos.y + self.size - 2.0, (self.size - 4.0) / 2.0, FUEL_COLOR);

        // Highlight
        draw_circle(pos.x + self.size / 2.0 - 3.0, pos.y + 6.0, 3.0, Color::new(1.0, 0.8, 0.6, 0.8));

        // Border
        draw_rectangle_lines(pos.x, pos.y, self.size, self.size, 2.0, DARKGRAY);
    }
}

pub const MAX_CAPSULES: usize = 3;

pub struct FuelSystem {
    pub capsules: Vec<FuelCapsule>,
    pub current_fuel: f32,
    pub fuel_tank_pos: Vec2,
    pub capsules_delivered: usize,  // Nombre de capsules déjà livrées
}

impl FuelSystem {
    pub fn new(fuel_tank_pos: Vec2) -> Self {
        let mut system = Self {
            capsules: Vec::new(),
            current_fuel: 0.0,
            fuel_tank_pos,
            capsules_delivered: 0,
        };
        // Spawn la première capsule immédiatement
        system.spawn_capsule();
        system
    }

    /// Spawn a single fuel capsule at random position
    fn spawn_capsule(&mut self) {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let x = rand::gen_range(60.0, screen_w - 80.0);
        let y = rand::gen_range(80.0, screen_h - 150.0);
        self.capsules.push(FuelCapsule::new(vec2(x, y)));
    }

    /// Check if we should spawn the next capsule
    pub fn update_spawning(&mut self) {
        // Spawn next capsule only if:
        // - We haven't delivered all capsules yet
        // - No capsule currently exists (previous one was delivered)
        if self.capsules_delivered < MAX_CAPSULES && self.capsules.is_empty() {
            self.spawn_capsule();
        }
    }

    /// Get number of capsules remaining to deliver
    pub fn capsules_remaining(&self) -> usize {
        MAX_CAPSULES - self.capsules_delivered
    }

    /// Check if all capsules have been delivered
    pub fn all_delivered(&self) -> bool {
        self.capsules_delivered >= MAX_CAPSULES
    }

    /// Find carried capsule index
    pub fn find_carried(&self) -> Option<usize> {
        self.capsules.iter().position(|c| matches!(c.state, FuelState::Carried))
    }

    /// Check if any capsule is falling
    pub fn has_falling(&self) -> bool {
        self.capsules.iter().any(|c| matches!(c.state, FuelState::Falling { .. }))
    }

    /// Update falling capsules and collect fuel when landed
    /// Returns true if a capsule was delivered
    pub fn update_falling(&mut self) -> bool {
        let mut landed = false;
        let mut landed_indices = Vec::new();

        for (i, capsule) in self.capsules.iter_mut().enumerate() {
            if capsule.update_falling() {
                landed_indices.push(i);
                landed = true;
            }
        }

        // Add fuel for landed capsules, increment counter and remove them
        for i in landed_indices.iter().rev() {
            self.current_fuel += self.capsules[*i].amount;
            self.capsules_delivered += 1;
            self.capsules.remove(*i);
        }

        // Clamp to capacity
        if self.current_fuel > FUEL_CAPACITY {
            self.current_fuel = FUEL_CAPACITY;
        }

        landed
    }

    /// Check if fuel tank is full
    pub fn is_full(&self) -> bool {
        self.current_fuel >= FUEL_CAPACITY
    }

    pub fn draw(&self, player_pos: Option<Vec2>) {
        // Draw spawned capsules
        for capsule in &self.capsules {
            if matches!(capsule.state, FuelState::Spawned { .. }) {
                capsule.draw(None);
            }
        }

        // Draw falling capsules
        for capsule in &self.capsules {
            if matches!(capsule.state, FuelState::Falling { .. }) {
                capsule.draw(None);
            }
        }

        // Draw carried capsule
        for capsule in &self.capsules {
            if matches!(capsule.state, FuelState::Carried) {
                capsule.draw(player_pos);
            }
        }
    }

    /// Draw fuel gauge UI
    pub fn draw_gauge(&self, x: f32, y: f32, width: f32, height: f32) {
        // Background
        draw_rectangle(x, y, width, height, DARKGRAY);

        // Fill
        let fill_width = (self.current_fuel / FUEL_CAPACITY) * width;
        let fill_color = if self.is_full() {
            Color::new(0.2, 0.9, 0.3, 1.0) // Green when full
        } else {
            Color::new(0.9, 0.5, 0.1, 1.0) // Orange
        };
        draw_rectangle(x, y, fill_width, height, fill_color);

        // Border
        draw_rectangle_lines(x, y, width, height, 2.0, WHITE);

        // Text
        let text = format!("FUEL: {:.0}%", (self.current_fuel / FUEL_CAPACITY) * 100.0);
        draw_text(&text, x + 5.0, y + height - 3.0, 14.0, WHITE);
    }
}

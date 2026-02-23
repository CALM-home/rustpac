use macroquad::prelude::*;

pub const MODULE_SIZE: f32 = 28.0;
const MODULE_COLOR_BOTTOM: Color = Color::new(0.8, 0.2, 0.2, 1.0); // Rouge
const MODULE_COLOR_MIDDLE: Color = Color::new(0.8, 0.6, 0.2, 1.0); // Orange/Or
const MODULE_COLOR_TOP: Color = Color::new(0.9, 0.9, 0.9, 1.0);    // Blanc

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ModuleType {
    Bottom, // Base avec moteurs
    Middle, // Réservoir
    Top,    // Module spatial
}

#[derive(Clone, Copy, PartialEq)]
pub enum ModuleState {
    Scattered { pos: Vec2 },      // Par terre, à ramasser
    Carried,                      // Transporté par le joueur
    Falling { pos: Vec2, vel_y: f32, target_pos: Vec2 }, // En chute vers position
    Placed { pos: Vec2 },         // Positionné sur la fusée
}

const MODULE_GRAVITY: f32 = 0.15;
const MODULE_MAX_FALL_SPEED: f32 = 3.0;
const MODULE_HORIZONTAL_SPEED: f32 = 0.08; // Vitesse d'alignement horizontal

pub struct RocketModule {
    pub module_type: ModuleType,
    pub state: ModuleState,
    pub size: f32,
}

impl RocketModule {
    pub fn new(module_type: ModuleType, pos: Vec2) -> Self {
        Self {
            module_type,
            state: ModuleState::Scattered { pos },
            size: MODULE_SIZE,
        }
    }

    pub fn color(&self) -> Color {
        match self.module_type {
            ModuleType::Bottom => MODULE_COLOR_BOTTOM,
            ModuleType::Middle => MODULE_COLOR_MIDDLE,
            ModuleType::Top => MODULE_COLOR_TOP,
        }
    }

    pub fn get_position(&self) -> Option<Vec2> {
        match self.state {
            ModuleState::Scattered { pos } => Some(pos),
            ModuleState::Placed { pos } => Some(pos),
            ModuleState::Falling { pos, .. } => Some(pos),
            ModuleState::Carried => None,
        }
    }

    pub fn set_carried(&mut self) {
        self.state = ModuleState::Carried;
    }

    pub fn place(&mut self, pos: Vec2) {
        self.state = ModuleState::Placed { pos };
    }

    /// Start falling toward target position
    pub fn start_falling(&mut self, start_pos: Vec2, target_pos: Vec2) {
        self.state = ModuleState::Falling {
            pos: start_pos,
            vel_y: 0.0,
            target_pos,
        };
    }

    /// Update falling physics, returns true if landed
    pub fn update_falling(&mut self) -> bool {
        if let ModuleState::Falling { pos, vel_y, target_pos } = &mut self.state {
            // Apply gravity (vertical)
            *vel_y += MODULE_GRAVITY;
            if *vel_y > MODULE_MAX_FALL_SPEED {
                *vel_y = MODULE_MAX_FALL_SPEED;
            }

            // Move down
            pos.y += *vel_y;

            // Horizontal alignment: smoothly move toward target x
            let x_diff = target_pos.x - pos.x;
            pos.x += x_diff * MODULE_HORIZONTAL_SPEED;

            // Check if reached target (vertical position + close enough horizontally)
            if pos.y >= target_pos.y && x_diff.abs() < 1.0 {
                *pos = *target_pos; // Snap to exact position
                self.state = ModuleState::Placed { pos: *pos };
                return true;
            }
        }
        false
    }

    pub fn draw(&self, player_pos: Option<Vec2>) {
        let pos = match self.state {
            ModuleState::Scattered { pos } => pos,
            ModuleState::Placed { pos } => pos,
            ModuleState::Falling { pos, .. } => pos,
            ModuleState::Carried => {
                // Follow player if provided
                if let Some(p_pos) = player_pos {
                    vec2(p_pos.x, p_pos.y - self.size - 5.0)
                } else {
                    return;
                }
            }
        };

        // Main body
        draw_rectangle(pos.x, pos.y, self.size, self.size, self.color());
        
        // Border
        draw_rectangle_lines(pos.x, pos.y, self.size, self.size, 2.0, DARKGRAY);

        // Type indicator
        match self.module_type {
            ModuleType::Bottom => {
                // Engine detail
                draw_rectangle(pos.x + 4.0, pos.y + self.size - 6.0, self.size - 8.0, 4.0, BLACK);
            }
            ModuleType::Middle => {
                // Tank stripes
                draw_line(
                    pos.x + 4.0, pos.y + self.size / 2.0,
                    pos.x + self.size - 4.0, pos.y + self.size / 2.0,
                    2.0, DARKGRAY,
                );
            }
            ModuleType::Top => {
                // Window
                draw_circle(pos.x + self.size / 2.0, pos.y + self.size / 2.0, 6.0, DARKBLUE);
            }
        }
    }

    /// Check if player can pick up this module (vertical alignment + correct order)
    pub fn check_pickup(&self, player_pos: Vec2, player_size: f32, allowed_type: ModuleType) -> bool {
        // Must be the correct type in sequence
        if self.module_type != allowed_type {
            return false;
        }
        
        // Can't pick up if falling or already placed
        if matches!(self.state, ModuleState::Falling { .. }) || matches!(self.state, ModuleState::Placed { .. }) {
            return false;
        }
        
        if let ModuleState::Scattered { pos } = self.state {
            let player_center_x = player_pos.x + player_size / 2.0;
            let module_center_x = pos.x + self.size / 2.0;
            let player_bottom = player_pos.y + player_size;
            let module_top = pos.y;
            
            // Vertical alignment: player must be directly above module
            let x_aligned = (player_center_x - module_center_x).abs() < self.size * 0.6;
            // Vertical proximity: player bottom near module top
            let y_close = player_bottom >= module_top - 5.0 && player_bottom <= module_top + self.size;
            
            return x_aligned && y_close;
        }
        false
    }
}

pub struct Rocket {
    pub modules: Vec<RocketModule>,
    pub base_position: Vec2, // Where the rocket should be assembled
}

impl Rocket {
    pub fn new(base_x: f32, base_y: f32) -> Self {
        Self {
            modules: Vec::new(),
            base_position: vec2(base_x, base_y),
        }
    }

    pub fn spawn_modules(&mut self, screen_width: f32, screen_height: f32) {
        self.modules.clear();
        
        // Fixed positions for scattered modules - NOT above the base (center)
        // so player has to fly horizontally to place them
        let positions = vec![
            vec2(80.0, screen_height - 200.0),                    // Bottom: bottom-left
            vec2(screen_width - 120.0, screen_height - 180.0),    // Middle: bottom-right  
            vec2(screen_width / 4.0, 120.0),                      // Top: upper-left (NOT center!)
        ];

        self.modules.push(RocketModule::new(ModuleType::Bottom, positions[0]));
        self.modules.push(RocketModule::new(ModuleType::Middle, positions[1]));
        self.modules.push(RocketModule::new(ModuleType::Top, positions[2]));
    }

    pub fn is_complete(&self) -> bool {
        self.modules.iter().all(|m| matches!(m.state, ModuleState::Placed { .. }))
    }

    pub fn get_placement_pos(&self, module_type: ModuleType) -> Vec2 {
        let offset = match module_type {
            ModuleType::Bottom => 0.0,
            ModuleType::Middle => MODULE_SIZE,
            ModuleType::Top => MODULE_SIZE * 2.0,
        };
        vec2(self.base_position.x, self.base_position.y - offset)
    }

    /// Get next module type that should be picked up (same as placement order)
    pub fn get_next_module_type(&self) -> Option<ModuleType> {
        // Must collect bottom first, then middle, then top
        if !self.modules.iter().any(|m| m.module_type == ModuleType::Bottom && 
            (matches!(m.state, ModuleState::Placed { .. }) || matches!(m.state, ModuleState::Carried))) {
            return Some(ModuleType::Bottom);
        }
        if !self.modules.iter().any(|m| m.module_type == ModuleType::Middle && 
            (matches!(m.state, ModuleState::Placed { .. }) || matches!(m.state, ModuleState::Carried))) {
            return Some(ModuleType::Middle);
        }
        if !self.modules.iter().any(|m| m.module_type == ModuleType::Top && 
            (matches!(m.state, ModuleState::Placed { .. }) || matches!(m.state, ModuleState::Carried))) {
            return Some(ModuleType::Top);
        }
        None
    }

    pub fn get_next_placement_type(&self) -> Option<ModuleType> {
        // Same logic but only for placed modules
        self.get_next_module_type()
    }

    /// Check if player is in the assembly column (X alignment with rocket base)
    /// Tolerance: player center within ~15 pixels of base center
    pub fn is_in_assembly_column(&self, player_pos: Vec2, player_size: f32) -> bool {
        let player_center_x = player_pos.x + player_size / 2.0;
        let base_center_x = self.base_position.x + MODULE_SIZE / 2.0;
        (player_center_x - base_center_x).abs() < 15.0
    }

    pub fn can_place_at_base(&self, player_pos: Vec2, player_size: f32) -> bool {
        if let Some(_next_type) = self.get_next_placement_type() {
            return self.is_in_assembly_column(player_pos, player_size);
        }
        false
    }

    pub fn draw(&self, player_pos: Option<Vec2>) {
        // Draw placed modules and scattered modules
        for module in &self.modules {
            if matches!(module.state, ModuleState::Placed { .. }) || matches!(module.state, ModuleState::Scattered { .. }) {
                module.draw(None);
            }
        }

        // Draw falling modules
        for module in &self.modules {
            if matches!(module.state, ModuleState::Falling { .. }) {
                module.draw(None);
            }
        }

        // Draw carried module (follows player)
        for module in &self.modules {
            if matches!(module.state, ModuleState::Carried) {
                module.draw(player_pos);
            }
        }

        // Draw ghost/outline for next placement position (only if not falling)
        let has_falling = self.modules.iter().any(|m| matches!(m.state, ModuleState::Falling { .. }));
        if !has_falling {
            if let Some(next_type) = self.get_next_placement_type() {
                let pos = self.get_placement_pos(next_type);
                draw_rectangle_lines(pos.x, pos.y, MODULE_SIZE, MODULE_SIZE, 2.0, Color::new(1.0, 1.0, 1.0, 0.3));
            }
        }
    }

    /// Update all falling modules, returns true if any landed
    pub fn update_falling(&mut self) -> bool {
        let mut any_landed = false;
        for module in &mut self.modules {
            if module.update_falling() {
                any_landed = true;
            }
        }
        any_landed
    }

    /// Check if any module is currently falling
    pub fn has_falling_module(&self) -> bool {
        self.modules.iter().any(|m| matches!(m.state, ModuleState::Falling { .. }))
    }
}

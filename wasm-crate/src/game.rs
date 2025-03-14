use std::ops::Div;

use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use nalgebra::Vector2;
use crate::spike::Spike;

#[wasm_bindgen]
pub struct Game {
    position: Vector2<f64>,
    velocity: Vector2<f64>,
    gravity: f64,
    is_grounded: bool,
    spikes: Vec<Spike>,
    spawn_timer: f64,
    canvas_width: f64,
    canvas_height: f64,
    game_over: bool,
    player_height: f64,
    player_width: f64,
    score: f64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        console_error_panic_hook::set_once();
        Game {
            position: Vector2::new(50.0, 80.0), 
            velocity: Vector2::new(0.0, 0.0),
            gravity: 0.5,
            is_grounded: true,
            spikes: Vec::new(),
            spawn_timer: 0.0,
            canvas_width: 800.0,
            canvas_height: 400.0,
            game_over: false,
            player_height: 20.0,
            player_width: 20.0,
            score: 0.0,
        }
    }
    
    pub fn update(&mut self) {
        self.velocity.y += self.gravity;
        
        self.position += self.velocity;
        
        if self.position.y + 20.0 >= 400.0 {
            self.position.y = 380.0;
            self.velocity.y = 0.0;
            self.is_grounded = true;
        } else {
            self.is_grounded = false;
        }
        
        web_sys::console::log_1(&format!("Position: ({}, {}), Velocity: ({}, {}), Grounded: {}", 
            self.position.x, self.position.y, self.velocity.x, self.velocity.y, self.is_grounded).into());

        // Update spikes
        for spike in &mut self.spikes {
            spike.update();
        }

        // Remove off-screen spikes
        self.spikes.retain(|spike| spike.is_visible());

        // Spawn new spikes
        self.spawn_timer += 1.0;
        self.score += 1.0;
        // web_sys::console::log_1(&format!("Spawn timer: {}", self.spawn_timer).into());
        if self.spawn_timer >= 120.0 { // Spawn every 120 frames
            web_sys::console::log_1(&"Attempting to spawn spike".into());
            self.spawn_spike();
            self.spawn_timer = 0.0;
        }

        // Check for spike collisions
        if self.check_spike_collisions() {
            self.game_over = true;
        }
    }


    pub fn jump(&mut self) {
        // web_sys::console::log_1(&"Jump called".into());
        if self.is_grounded {
            // web_sys::console::log_1(&"Jumping!".into());
            self.velocity.y = -10.0; 
        } else {
            web_sys::console::log_1(&"Not grounded, can't jump".into());
        }
    }

    pub fn render(&self, crc: &CanvasRenderingContext2d) {
        crc.clear_rect(0.0, 0.0, 800.0, 400.0);
        
        // Draw ground
        crc.set_fill_style_str(&"black");
        crc.fill_rect(0.0, 400.0, 800.0, 20.0);
        
        // Render spikes
        for spike in &self.spikes {
            spike.render(crc);
        }
        
        // Draw character block
        crc.set_fill_style_str(&"red");
        crc.fill_rect(self.position.x, self.position.y, 20.0, 20.0);
        
        // Small text above the player to show the position
        crc.set_fill_style_str(&"black");
        let _ = crc.fill_text(&format!("({:.1}, {:.1})", self.position.x, self.position.y), self.position.x, self.position.y - 5.0);
    }

    pub fn spawn_spike(&mut self) {
        let mut rng = rand::rng();

        let spike_width = rng.random_range(10.0..30.0);
        let spike_height = rng.random_range(10.0..30.0);
        let spike_speed = self.score.div(100.0); // need to make change with survivaal time
        let new_spike = Spike::new(
            self.canvas_width, 
            self.canvas_height - spike_height,
            spike_width,
            spike_height,
            spike_speed
        );
        self.spikes.push(new_spike);

        web_sys::console::log_1(&format!("Spike spawned: width={}, height={}, speed={}", 
            spike_width, spike_height, spike_speed).into());
    }

    // I think this is like bounding box collision detection, I hope...
    pub fn check_spike_collisions(&mut self) -> bool {
        for spike in &mut self.spikes {
            if spike.is_visible() {
                let spike_pos = spike.position();
                if self.position.x + self.player_width > spike_pos[0] &&
                    self.position.x < spike_pos[0] + spike.get_width() &&
                    self.position.y + self.player_height > spike_pos[1] &&
                    self.position.y < spike_pos[1] + spike.get_height() {
                        return true; 
            }
            }
        }
        false
    }
    
}

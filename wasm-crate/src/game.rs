use crate::spike::Spike;
use nalgebra::Vector2;
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

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
            gravity: 0.4,
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

    pub fn update(&mut self, delta_time: f64) {
        if self.game_over {
            return;
        }

        self.velocity.y += self.gravity;
        self.position += self.velocity;

        if self.position.y + 20.0 >= 400.0 {
            self.position.y = 380.0;
            self.velocity.y = 0.0;
            self.is_grounded = true;
        } else {
            self.is_grounded = false;
        }

        for spike in &mut self.spikes {
            spike.update();
        }

        self.spikes.retain(|spike| spike.is_visible());

        self.spawn_timer += 1.0;
        self.score += delta_time * 60.0;

        let difficulty = 2.0;
        let spawn_interval =
            ((200.0 / difficulty) - (self.score / (100.0 / difficulty))).max(60.0 / difficulty);

        if self.spawn_timer >= spawn_interval {
            web_sys::console::log_1(&"Attempting to spawn spike".into());
            self.spawn_spike();
            self.spawn_timer = 0.0;
        }

        if self.check_spike_collisions() {
            self.game_over = true;
        }
    }

    pub fn jump(&mut self) {
        if self.is_grounded {
            self.velocity.y = -10.0;
        } else {
            web_sys::console::log_1(&"not grounded cant jump".into());
        }
    }

    pub fn render(&self, crc: &CanvasRenderingContext2d) {
        crc.clear_rect(0.0, 0.0, 800.0, 400.0);

        crc.set_fill_style_str(&"black");
        crc.fill_rect(0.0, 400.0, 800.0, 20.0);

        if !self.game_over {
            for spike in &self.spikes {
                spike.render(crc);
            }
        }

        crc.set_fill_style_str(&"red");
        crc.fill_rect(self.position.x, self.position.y, 20.0, 20.0);

        crc.set_fill_style_str(&"black");

        if self.game_over {
            crc.set_font("30px Arial");
            crc.set_fill_style_str("red");
            crc.set_text_align("center");
            let _ = crc.fill_text(
                "Game Over",
                self.canvas_width / 2.0,
                self.canvas_height / 2.0,
            );

            crc.set_font("20px Arial");
            let _ = crc.fill_text(
                &format!("Score: {:.0}", self.score),
                self.canvas_width / 2.0,
                self.canvas_height / 2.0 + 40.0,
            );

            crc.set_font("16px Arial");
            let _ = crc.fill_text(
                "Press 'R' to restart",
                self.canvas_width / 2.0,
                self.canvas_height / 2.0 + 70.0,
            );
        } else {
            crc.set_font("16px Arial");
            crc.set_fill_style_str("black");
            crc.set_text_align("left");
            let _ = crc.fill_text(&format!("Score: {:.0}", self.score), 10.0, 20.0);
        }
    }

    pub fn restart(&mut self) {
        if self.game_over {
            self.position = Vector2::new(50.0, 80.0);
            self.velocity = Vector2::new(0.0, 0.0);
            self.is_grounded = true;
            self.spikes.clear();
            self.spawn_timer = 0.0;
            self.game_over = false;
            self.score = 0.0;
        }
    }

    pub fn spawn_spike(&mut self) {
        let mut rng = rand::rng();

        let spike_width = rng.random_range(10.0..50.0);
        let spike_height = rng.random_range(10.0..50.0);

        let base_speed = 3.0;
        let speed_increment = (self.score / 1000.0).min(5.0);
        let spike_speed = base_speed + speed_increment;

        let new_spike = Spike::new(
            self.canvas_width,
            self.canvas_height - spike_height,
            spike_width,
            spike_height,
            spike_speed,
        );
        self.spikes.push(new_spike);

        web_sys::console::log_1(
            &format!(
                "Spike spawned: width={}, height={}, speed={}",
                spike_width, spike_height, spike_speed
            )
            .into(),
        );
    }

    // I think this is like bounding box collision detection, I hope it works...
    pub fn check_spike_collisions(&mut self) -> bool {
        for spike in &mut self.spikes {
            if spike.is_visible() {
                let spike_pos = spike.position();
                if self.position.x + self.player_width > spike_pos[0]
                    && self.position.x < spike_pos[0] + spike.get_width()
                    && self.position.y + self.player_height > spike_pos[1]
                    && self.position.y < spike_pos[1] + spike.get_height()
                {
                    return true;
                }
            }
        }
        false
    }
}

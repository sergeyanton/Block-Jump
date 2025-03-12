use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use nalgebra::Vector2;


#[wasm_bindgen]
pub struct Spike {
    position: Vector2<f64>,
    width: f64,
    height: f64,
    speed: f64,
}
// asd
#[wasm_bindgen]
impl Spike {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64, speed: f64) -> Spike {
        Spike {
            position: Vector2::new(x, y),
            width,
            height,
            speed,
        }
    }

    pub fn update(&mut self) {
        self.position.x -= self.speed;
    }

    pub fn is_visible(&self) -> bool {
        self.position.x + self.width > 0.0
    }

    pub fn render(&self, crc: &CanvasRenderingContext2d) {
        crc.set_fill_style_str(&"green");
        
        // Draw a triangle for the spike
        crc.begin_path();
        crc.move_to(self.position.x, self.position.y + self.height);
        crc.line_to(self.position.x + self.width / 2.0, self.position.y);
        crc.line_to(self.position.x + self.width, self.position.y + self.height);
        crc.close_path();
        crc.fill();
    }

    pub fn position(&self) -> Vec<f64> {
        vec![self.position.x, self.position.y]
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }
}
mod game;
mod spike;

pub use game::Game;
pub use spike::Spike;

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
// main function for wasm to instantiate the library and call run

use self::graphics::Graphics;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn main() {
    let graphics = Graphics::new();
    graphics.run();
}
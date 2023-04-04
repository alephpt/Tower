use gfx::graphics::*;

fn main() {
    let graphics = Graphics::new();
    pollster::block_on(graphics.run());
}

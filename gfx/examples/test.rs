use gfx::graphics::*;

fn main() {
    let _ = pollster::block_on(Graphics::run());
}

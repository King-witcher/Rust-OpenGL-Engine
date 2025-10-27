extern crate gl46;
extern crate sdl2;

mod engine;
mod shader_program;
mod window;

fn main() {
    let engine = engine::KEngine::new(800, 600, "Rust OpenGL Window");
    engine.run();
}

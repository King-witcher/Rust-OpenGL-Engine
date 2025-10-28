extern crate gl46 as gl;
extern crate sdl2;

mod engine;
mod model;
mod shader_program;
mod window;

fn main() {
    let engine = engine::KEngine::new(800, 600, "Rust OpenGL Window");
    engine.run();
}

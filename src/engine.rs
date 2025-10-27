use crate::window;
use gl46::*;
use std::{ffi::CStr, rc::Rc};

pub struct KEngine {
    window: window::KWindow,
    gl: Rc<GlFns>,
}

impl KEngine {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let window = window::KWindow::new(window::KWindowCreateInfo {
            title,
            width,
            height,
        });

        let load_function = |s: *const u8| unsafe {
            let str = CStr::from_ptr(s as _)
                .to_str()
                .expect("Failed to convert CStr");
            window.get_proc_address(str)
        };

        let gl = unsafe { GlFns::load_from(&load_function) };
        let gl = gl.expect("Failed to load OpenGL functions");

        KEngine {
            window,
            gl: Rc::new(gl),
        }
    }

    pub fn run(&self) {
        unsafe {
            let mut event_pump = self.window.event_pump();
            self.gl.ClearColor(0.1, 0.2, 0.3, 1.0);

            'main_loop: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit { .. } => break 'main_loop,
                        _ => {}
                    }
                }

                self.gl.Clear(GL_COLOR_BUFFER_BIT);
                self.window.swap_window();
            }
        }
    }
}

use std::rc::Rc;

mod shader;

pub struct ShaderProgram {
    gl: Rc<gl46::GlFns>,
    program_id: u32,
}

pub struct ShaderProgramCreateInfo<'s> {
    pub gl: Rc<gl46::GlFns>,
    pub vertex_source: &'s str,
    pub fragment_source: &'s str,
    pub source_type: shader::ShaderSourceType,
}

impl ShaderProgram {
    pub fn new(create_info: ShaderProgramCreateInfo) -> Self {
        // Shader compilation and linking logic would go here.
        // For brevity, this is left as a placeholder.

        ShaderProgram {
            program_id: 0,
            gl: create_info.gl,
        }
    }

    pub fn use_program(&self) {
        self.gl.UseProgram(self.program_id);
    }
}

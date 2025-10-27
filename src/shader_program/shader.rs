use std::{fs, rc::Rc};

use gl46::*;

pub struct Shader {
    shader_id: u32,
    gl: Rc<gl46::GlFns>,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ShaderSourceType {
    GLSL,
    SPIRV,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ShaderType {
    Vertex = GL_VERTEX_SHADER.0,
    Fragment = GL_FRAGMENT_SHADER.0,
}

pub struct ShaderCreateInfo<'s> {
    pub gl: Rc<gl46::GlFns>,
    pub path: &'s str,
    pub source_type: ShaderSourceType,
    pub shader_type: ShaderType,
}

impl Shader {
    pub fn new(create_info: ShaderCreateInfo) -> Self {
        let ShaderCreateInfo {
            gl,
            path,
            shader_type,
            source_type,
        } = create_info;

        match source_type {
            ShaderSourceType::GLSL => Self::create_glsl(gl.clone(), path, shader_type),
            ShaderSourceType::SPIRV => Self::create_spv(gl.clone(), path, shader_type),
        }
    }

    pub fn id(&self) -> u32 {
        self.shader_id
    }

    fn create_glsl(gl: Rc<GlFns>, path: &str, shader_type: ShaderType) -> Self {
        unsafe {
            let glsl = read_text_file(path);
            let glsl_ptr = glsl.as_ptr() as _;
            let glsl_len = glsl.len() as _;

            let shader_id = gl.CreateShader(GLenum(shader_type as _));
            gl.ShaderSource(shader_id, 1, &glsl_ptr, &glsl_len);
            gl.CompileShader(shader_id);

            let mut success: i32 = 0;
            gl.GetShaderiv(shader_id, GL_COMPILE_STATUS, &mut success);

            if success == 0 {
                let mut length = 0;
                let mut info_log = [0u8; 512];
                gl.GetShaderInfoLog(shader_id, 512, &mut length, info_log.as_mut_ptr());
                panic!(
                    "Failed to compile shader: {}",
                    std::str::from_utf8(&info_log[..length as usize]).unwrap()
                );
            }

            Self { gl, shader_id }
        }
    }

    fn create_spv(gl: Rc<GlFns>, path: &str, shader_type: ShaderType) -> Self {
        unsafe {
            let spir_v = read_binary_file(path);

            let shader_id = gl.CreateShader(GLenum(shader_type as u32));
            gl.ShaderBinary(
                1,                              // How many shader ids
                &shader_id,                     // Pointer to shaderIds
                GL_SHADER_BINARY_FORMAT_SPIR_V, // GL_SHADER_BINARY_FORMAT_SPIR_V
                spir_v.as_ptr() as _,           // Pointer to binary data
                spir_v.len() as _,              // Length of binary data
            );
            gl.SpecializeShader(
                shader_id,
                b"main\0".as_ptr() as _,
                0,
                std::ptr::null(),
                std::ptr::null(),
            );

            let mut success: i32 = 0;
            gl.GetShaderiv(shader_id, GL_SPIR_V_BINARY, &mut success);

            if success == 0 {
                let mut length = 0;
                let mut info_log = [0u8; 512];
                gl.GetShaderInfoLog(shader_id, 512, &mut length, info_log.as_mut_ptr());
                panic!(
                    "Failed to specialize shader: {}",
                    std::str::from_utf8(&info_log[..length as usize]).unwrap()
                );
            }

            Self { gl, shader_id }
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.DeleteShader(self.shader_id);
    }
}

fn read_text_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read shader file")
}

fn read_binary_file(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read binary shader file")
}

use std::rc::Rc;

use glow::HasContext;

use thiserror::Error;

pub struct Shader {
    gl: Rc<glow::Context>,
    program: glow::NativeProgram,
}

#[repr(u32)]
pub enum ShaderType {
    VertexShader = glow::VERTEX_SHADER,
    FragmentShader = glow::FRAGMENT_SHADER,
}

#[derive(Error, Debug)]
pub enum CreationError {
    #[error("Failed to create shader: {error_message}")]
    ShaderCreationFailed { error_message: String },

    #[error("Failed to compile shader: {error_message}")]
    ShaderCompilationFailed { error_message: String },

    #[error("Failed to create program: {error_message}")]
    ProgramCreationFailed { error_message: String },

    #[error("Failed to compile program: {error_message}")]
    ProgramCompilationFailed { error_message: String },
}

impl Shader {
    pub fn from(
        gl: Rc<glow::Context>,
        vertex_shader: &str,
        fragment_shader: &str,
    ) -> Result<Self, CreationError> {
        // Vertex Shader
        let compiled_vertex_shader =
            Self::compile_shader(Rc::clone(&gl), ShaderType::VertexShader, vertex_shader)?;

        // Fragment Shader
        let compiled_fragment_shader =
            Self::compile_shader(Rc::clone(&gl), ShaderType::FragmentShader, fragment_shader)?;

        unsafe {
            let cleanup = || {
                // Delete shaders
                gl.delete_shader(compiled_vertex_shader);
                gl.delete_shader(compiled_fragment_shader);
            };
            // Program
            let program = match gl.create_program() {
                Ok(p) => p,
                Err(err) => {
                    cleanup();
                    return Err(CreationError::ProgramCreationFailed { error_message: err });
                }
            };

            gl.attach_shader(program, compiled_vertex_shader);
            gl.attach_shader(program, compiled_fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                cleanup();
                return Err(CreationError::ProgramCompilationFailed {
                    error_message: gl.get_program_info_log(program),
                });
            }

            // Unlink shaders then delete
            gl.detach_shader(program, compiled_vertex_shader);
            gl.detach_shader(program, compiled_fragment_shader);

            cleanup();

            Ok(Shader {
                gl: Rc::clone(&gl),
                program,
            })
        }
    }

    fn compile_shader(
        gl: Rc<glow::Context>,
        shader_type: ShaderType,
        source: &str,
    ) -> Result<glow::NativeShader, CreationError> {
        unsafe {
            let shader = match gl.create_shader(shader_type as u32) {
                Ok(shader) => shader,
                Err(err) => return Err(CreationError::ShaderCreationFailed { error_message: err }),
            };

            gl.shader_source(shader, source);
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                Err(CreationError::ShaderCompilationFailed {
                    error_message: gl.get_shader_info_log(shader),
                })
            } else {
                Ok(shader)
            }
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.use_program(Some(self.program)) }
    }

    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe {
            self.gl.uniform_1_i32(
                self.gl.get_uniform_location(self.program, name).as_ref(),
                value as i32,
            )
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        unsafe {
            self.gl.uniform_1_i32(
                self.gl.get_uniform_location(self.program, name).as_ref(),
                value,
            )
        }
    }
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe {
            self.gl.uniform_1_f32(
                self.gl.get_uniform_location(self.program, name).as_ref(),
                value,
            )
        }
    }

    pub fn set_mat4_float(&self, name: &str, transpose: bool, value: &[f32]) {
        // We should use an abstracted Mat4 type from glam or anouther library here
        // to remove the need to handle invalid values
        debug_assert!(value.len() == 16);
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                self.gl.get_uniform_location(self.program, name).as_ref(),
                transpose,
                value,
            )
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.program);
        }
    }
}

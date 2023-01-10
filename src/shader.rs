use std::rc::Rc;

use glow::HasContext;

use thiserror::Error;

use crate::texture::Texture;

pub struct Shader {
    gl: Rc<glow::Context>,
    program: glow::NativeProgram,
    textures: [Option<Texture>; 16],
}

#[repr(u32)]
pub enum ShaderType {
    VertexShader = glow::VERTEX_SHADER,
    FragmentShader = glow::FRAGMENT_SHADER,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureIndex {
    Index0 = 0,
    Index1,
    Index2,
    Index3,
    Index4,
    Index5,
    Index6,
    Index7,
    Index8,
    Index9,
    Index10,
    Index11,
    Index12,
    Index13,
    Index14,
    Index15,
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

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("Texture index already in use")]
    TextureIndexTaken { index: TextureIndex },
}

impl Shader {
    pub fn from_str(
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
                gl,
                program,
                textures: Default::default(),
            })
        }
    }

    pub fn add_texture(
        &mut self,
        texture: Texture,
        index: TextureIndex,
    ) -> Result<(), TextureError> {
        if self.textures[index as usize].is_some() {
            return Err(TextureError::TextureIndexTaken { index });
        }
        self.textures[index as usize] = Some(texture);
        Ok(())
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
        let texture_id = [
            glow::TEXTURE0,
            glow::TEXTURE1,
            glow::TEXTURE2,
            glow::TEXTURE3,
            glow::TEXTURE4,
            glow::TEXTURE5,
            glow::TEXTURE6,
            glow::TEXTURE7,
            glow::TEXTURE8,
            glow::TEXTURE9,
            glow::TEXTURE10,
            glow::TEXTURE11,
            glow::TEXTURE12,
            glow::TEXTURE13,
            glow::TEXTURE14,
            glow::TEXTURE15,
        ];
        unsafe {
            self.gl.use_program(Some(self.program));
            for (index, texture) in self.textures.iter().enumerate() {
                if let Some(texture) = texture {
                    self.gl.active_texture(texture_id[index]);
                    texture.bind();
                }
            }
        }
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

    pub fn set_vec3(&self, name: &str, value: glam::Vec3) {
        unsafe {
            self.gl.uniform_3_f32_slice(
                self.gl.get_uniform_location(self.program, name).as_ref(),
                &value.to_array(),
            )
        }
    }

    pub fn set_mat4(&self, name: &str, transpose: bool, value: &glam::Mat4) {
        unsafe {
            self.gl.uniform_matrix_4_f32_slice(
                self.gl.get_uniform_location(self.program, name).as_ref(),
                transpose,
                &value.to_cols_array(),
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

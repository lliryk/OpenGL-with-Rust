use std::{
    ffi::OsString,
    io::{self, BufReader},
    path::{Path, PathBuf},
    rc::Rc,
};

use glow::HasContext;
use image::DynamicImage;
use thiserror::Error;

pub struct Texture {
    gl: Rc<glow::Context>,
    texture: glow::NativeTexture,
}

#[derive(Error, Debug)]
pub enum CreationError {
    #[error("No file extension")]
    NoFileExtension { path: PathBuf },

    #[error("Unknown file extension: {extension:?}")]
    UnknownFileExtension { path: PathBuf, extension: OsString },

    #[error("Failed to open file: {error_message}")]
    FileOpenFailed {
        error_message: String,
        file_path: PathBuf,
        io_error: io::Error,
    },

    #[error("Failed to load image: {error}")]
    ImageLoadingFailed {
        path: PathBuf,
        extension: image::ImageFormat,
        error: image::ImageError,
    },

    #[error("Failed to create texture: {error_message}")]
    TextureCreationFailed {
        path: PathBuf,
        extension: image::ImageFormat,
        error_message: String,
    },
}

impl Texture {
    pub fn new(gl: Rc<glow::Context>, path: &Path) -> Result<Self, CreationError> {
        let format = match path.extension() {
            Some(ext) => match image::ImageFormat::from_extension(ext) {
                Some(format) => format,
                None => {
                    return Err(CreationError::UnknownFileExtension {
                        path: PathBuf::from(path),
                        extension: ext.to_os_string(),
                    })
                }
            },
            None => {
                return Err(CreationError::NoFileExtension {
                    path: PathBuf::from(path),
                })
            }
        };

        let file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                return Err(CreationError::FileOpenFailed {
                    error_message: err.to_string(),
                    file_path: PathBuf::from(path),
                    io_error: err,
                })
            }
        };

        let buffer = BufReader::new(file);

        let image = match image::load(buffer, format) {
            Ok(image) => image.flipv(),
            Err(err) => {
                return Err(CreationError::ImageLoadingFailed {
                    path: PathBuf::from(path),
                    extension: format,
                    error: err,
                })
            }
        };

        unsafe {
            let texture = match gl.create_texture() {
                Ok(texture) => texture,
                Err(err) => {
                    return Err(CreationError::TextureCreationFailed {
                        path: PathBuf::from(path),
                        extension: format,
                        error_message: err,
                    })
                }
            };

            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            let image = DynamicImage::ImageRgba8(image.into_rgba8());

            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32, // Thanks OpenGL
                image.width() as i32,
                image.height() as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(image.as_bytes()),
            );

            gl.generate_mipmap(glow::TEXTURE_2D);
            Ok(Texture { gl, texture })
        }
    }

    pub fn bind(&self) {
        unsafe { self.gl.bind_texture(glow::TEXTURE_2D, Some(self.texture)) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { self.gl.delete_texture(self.texture) }
    }
}

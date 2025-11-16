use std::{ffi::CString, fs::read_to_string, ptr};

use gl::types::{GLenum, GLint, GLuint};

use crate::shaders::ShaderError;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub unsafe fn new(source_code: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code)?;

        // Create a new OpenGL shader.
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };

        // Create and compile the shader source code.
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        // check for shader compilation errors
        let mut success: GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            Ok(shader)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::CompilationError(log))
        }
    }

    pub unsafe fn from_file(path: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let content = match read_to_string(path) {
            Ok(x) => x,
            Err(_) => return Err(ShaderError::FileNotFoundError),
        };

        Self::new(content.as_str(), shader_type)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

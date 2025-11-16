use std::ffi::{CString, NulError};

use gl::types::{GLint, GLuint};

use crate::shaders::{Shader, ShaderError};

pub struct ShaderProgram {
    pub id: GLuint,
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        // Create the OpenGL shader program.
        let program = Self {
            id: gl::CreateProgram(),
        };

        // Add all the shaders to the program.
        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        // Link all the shaders together.
        gl::LinkProgram(program.id);

        // Check for errors.
        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        // Tell OpenGL which shader program we want to use.
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError> {
        let attrib = CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }

    pub unsafe fn get_uniform_attrib_location(&self, attrib: &str) -> Result<GLint, NulError> {
        let attrib = CString::new(attrib)?;
        Ok(gl::GetUniformLocation(self.id, attrib.as_ptr()))
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

mod buffer;
mod shader;
mod shader_program;
mod texture;
mod vertex_array;

use std::{ffi::NulError, string::FromUtf8Error};

pub use buffer::*;
pub use shader::*;
pub use shader_program::*;
pub use texture::*;
pub use vertex_array::*;

pub type Pos = [f32; 2];
pub type Pos3 = [f32; 3];
pub type Color = [f32; 3];
pub type TextureCoords = [f32; 2];

#[derive(Debug)]
pub enum ShaderError {
    CompilationError(String),
    LinkingError(String),
    FileNotFoundError,
    OtherError,
}

impl From<FromUtf8Error> for ShaderError {
    fn from(_: FromUtf8Error) -> ShaderError {
        ShaderError::OtherError
    }
}

impl From<NulError> for ShaderError {
    fn from(_: NulError) -> ShaderError {
        ShaderError::OtherError
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}

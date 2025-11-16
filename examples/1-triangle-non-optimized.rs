use std::{ffi::CString, ptr};

use gl::types::{GLsizeiptr, GLuint};
use glutin::{Api, ContextBuilder, GlRequest, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use opengl_test::shaders::Pos3;

// Tells OpenGL where the triangle needs to be positioned.
const VERTEX_SHADER_SOURCE: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

// Tells OpenGL what color each position needs to be.
const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(0.8f, 0.3f, 0.02f, 1.0f);
}
"#;

#[repr(C, packed)]
struct Vertex(Pos3);

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("OpenGL in Rust");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let vertices: [Vertex; 3] = [
        Vertex([-0.5, -0.5 * (3.0f32).sqrt() / 3.0, 0.0]),
        Vertex([0.5,  -0.5 * (3.0f32).sqrt() / 3.0, 0.0]),
        Vertex([0.0,   0.5 * (3.0f32).sqrt() * 2.0 / 3.0, 0.0])
    ];

    let mut vao: GLuint;
    let mut vbo: GLuint;
    let program: GLuint;
    unsafe {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let vertex_source_code = CString::new(VERTEX_SHADER_SOURCE).unwrap();
        gl::ShaderSource(vertex_shader, 1, &vertex_source_code.as_ptr(), ptr::null());
        gl::CompileShader(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let fragment_source_code = CString::new(FRAGMENT_SHADER_SOURCE).unwrap();
        gl::ShaderSource(fragment_shader, 1, &fragment_source_code.as_ptr(), ptr::null());
        gl::CompileShader(fragment_shader);

        program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        // Delete shaders because they are in the program now
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        vao = 0;
        gl::GenVertexArrays(1, &mut vao);

        vbo = 0;
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        let (_, data_bytes, _) = vertices.align_to::<u8>();
        gl::BufferData(
            gl::ARRAY_BUFFER,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            gl::STATIC_DRAW
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * std::mem::size_of::<f32>() as i32, ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                unsafe {
                    gl::DeleteVertexArrays(1, &vao);
                    gl::DeleteBuffers(1, &vbo);
                    gl::DeleteProgram(program);
                }
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // call main redraw function

                unsafe {
                    gl::ClearColor(0.07, 0.13, 0.17, 1.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                    gl::UseProgram(program);
                    gl::BindVertexArray(vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);
                }

                // at the end of the full draw, swap the draw buffer with the current buffer
                gl_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

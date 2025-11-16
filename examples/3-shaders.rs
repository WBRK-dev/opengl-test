use std::{ffi::CString, ptr, time::Instant};
use gl::types::{GLint, GLuint};
use glutin::{Api, ContextBuilder, GlRequest, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use opengl_test::{set_attribute, shaders::{Buffer, Color, Pos3, Shader, ShaderProgram, VertexArray}};

#[repr(C, packed)]
struct Vertex(Pos3, Color);

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("OpenGL in Rust");

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let vertices: [Vertex; 6] = [
        Vertex([-0.5, -0.5 * (3.0f32).sqrt() / 3.0, 0.0], [0.1, 0.2, 0.3]),
        Vertex([0.5, -0.5 * (3.0f32).sqrt() / 3.0, 0.0], [0.9, 0.2, 0.9]),
        Vertex([0.0, 0.5 * (3.0f32).sqrt() * 2.0 / 3.0, 0.0], [0.4, 0.5, 0.3]),
        Vertex([-0.5 / 2.0, 0.5 * (3.0f32).sqrt() / 6.0, 0.0], [0.9, 0.2, 0.3]),
        Vertex([0.5 / 2.0, 0.5 * (3.0f32).sqrt() / 6.0, 0.0], [0.1, 0.6, 0.3]),
        Vertex([0.0, -0.5 * (3.0f32).sqrt() / 3.0, 0.0], [0.1, 0.2, 0.9]),
    ];

    let indices: [GLuint; 9] = [
        0, 3, 5,  // Lower left
        3, 2, 4,  // Lower right
        5, 4, 1,  // Upper
    ];

    let program;
    let _vertex_buffer;
    let _indeces_buffer;
    let vertex_array;
    let uniform_id: GLint;
    unsafe {
        // Create the two shaders.
        let vertex_shader = Shader::from_file("assets/shaders/default.vert", gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::from_file("assets/shaders/default.frag", gl::FRAGMENT_SHADER).unwrap();
        program = ShaderProgram::new(&[vertex_shader, fragment_shader]).unwrap();

        vertex_array = VertexArray::new();
        vertex_array.bind();

        // Send the vertices to the GPU.
        _vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        _vertex_buffer.set_data(&vertices, gl::STATIC_DRAW);

        // Send the indices to the GPU.
        _indeces_buffer = Buffer::new(gl::ELEMENT_ARRAY_BUFFER);
        _indeces_buffer.set_data(&indices, gl::STATIC_DRAW);

        // Tell the GPU what data to use for the position of the vertices.
        let pos_attrib = program.get_attrib_location("position").unwrap();
        set_attribute!(vertex_array, pos_attrib, Vertex::0);
        let pos_attrib = program.get_attrib_location("color").unwrap();
        set_attribute!(vertex_array, pos_attrib, Vertex::1);

        let location = CString::new("scale").unwrap();
        uniform_id = gl::GetUniformLocation(program.id, location.as_ptr());  // Get the location of the uniform parameter in the shader.

        // Bind the VBO, VAO and EBO to 0 so we dont modify it.
        _vertex_buffer.unbind();
        vertex_array.unbind();
        _indeces_buffer.unbind();
    }

    let mut scale: f32 = 0.5;
    let mut last_scale_update = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;


        if last_scale_update.elapsed().as_secs_f32() >= 1f32 / 60f32 {
            scale += 0.005f32;
            if scale > 1.5 {
                scale = 0.5;
            }
            last_scale_update = Instant::now();
            gl_context.window().request_redraw();
        }

        match event {
            Event::LoopDestroyed => (),
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
                    program.apply();  // Tell OpenGL to use this shader program.
                    gl::Uniform1f(uniform_id, scale);  // Set the uniform value.
                    vertex_array.bind();  // Bind the VERTICES so OpenGL uses it.
                    gl::DrawElements(gl::TRIANGLES, 9, gl::UNSIGNED_INT, ptr::null());  // Draw the triangle using GL_TRIANGLES primitive.
                }

                // at the end of the full draw, swap the draw buffer with the current buffer
                gl_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

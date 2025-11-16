use glutin::{Api, ContextBuilder, GlRequest, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use opengl_test::{set_attribute, shaders::{Buffer, Pos3, Shader, ShaderProgram, VertexArray}};

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

// Positions of triangle
fn get_vertices() -> [Vertex; 3] {
    [
        Vertex([-0.5, -0.5 * (3.0f32).sqrt() / 3.0, 0.0]),
        Vertex([0.5,  -0.5 * (3.0f32).sqrt() / 3.0, 0.0]),
        Vertex([0.0,   0.5 * (3.0f32).sqrt() * 2.0 / 3.0, 0.0])
    ]
}

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

    let program;
    let _vertex_buffer;
    let vertex_array;
    unsafe {
        // Create the two shaders.
        let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER).unwrap();
        program = ShaderProgram::new(&[vertex_shader, fragment_shader]).unwrap();

        // Send the vertices to the GPU.
        _vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
        _vertex_buffer.set_data(&get_vertices(), gl::STATIC_DRAW);

        // Tell the GPU what data to use for the position of the vertices.
        vertex_array = VertexArray::new();
        let pos_attrib = program.get_attrib_location("aPos").unwrap();
        set_attribute!(vertex_array, pos_attrib, Vertex::0);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

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
                    program.apply();  // Tell OpenGL to use this shader program
                    vertex_array.bind();  // Bind the VERTICES so OpenGL uses it.
                    gl::DrawArrays(gl::TRIANGLES, 0, 3);  // Draw the triangle using GL_TRIANGLES primitive.
                }

                // at the end of the full draw, swap the draw buffer with the current buffer
                gl_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

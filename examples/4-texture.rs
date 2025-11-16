use std::{ffi::CString, path::Path, ptr};
use gl::types::{GLint, GLuint};
use glutin::{Api, ContextBuilder, GlRequest, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use opengl_test::{set_attribute, shaders::{Buffer, Color, Pos, Pos3, Shader, ShaderProgram, Texture, VertexArray}};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

#[repr(C, packed)]
struct Vertex(Pos3, Color, Pos);

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("OpenGL in Rust")
        .with_inner_size(glutin::dpi::PhysicalSize::new(WIDTH, HEIGHT));

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

    let vertices: [Vertex; 4] = [
        Vertex([-0.5, -0.5, 0.0], [0.1, 0.2, 0.3], [0.0, 0.0]),
        Vertex([-0.5, 0.5, 0.0], [0.9, 0.2, 0.9], [0.0, 1.0]),
        Vertex([0.5, 0.5, 0.0], [0.4, 0.5, 0.3], [1.0, 1.0]),
        Vertex([0.5, -0.5, 0.0], [0.1, 0.2, 0.9], [1.0, 0.0]),
    ];

    let indices: [GLuint; 6] = [
        0, 2, 1,
        0, 3, 2,
    ];

    let program;
    let _vertex_buffer;
    let _indeces_buffer;
    let vertex_array;
    let uniform_id: GLint;
    let texture;
    unsafe {
        // Create the two shaders.
        let vertex_shader = Shader::from_file("assets/shaders/4-texture.vert", gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::from_file("assets/shaders/4-texture.frag", gl::FRAGMENT_SHADER).unwrap();
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
        let pos_attrib = program.get_attrib_location("texture").unwrap();
        set_attribute!(vertex_array, pos_attrib, Vertex::2);

        // Bind the VBO, VAO and EBO to 0 so we dont modify it.
        _vertex_buffer.unbind();
        vertex_array.unbind();
        _indeces_buffer.unbind();

        let location = CString::new("scale").unwrap();
        uniform_id = gl::GetUniformLocation(program.id, location.as_ptr());  // Get the location of the uniform parameter in the shader.

        texture = Texture::new();
        gl::ActiveTexture(gl::TEXTURE0);
        texture.bind();

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        let _ = texture.load(&Path::new("assets/textures/grass_block.webp")).unwrap();

        let location = CString::new("tex0").unwrap();
        let texture_uni_id = gl::GetUniformLocation(program.id, location.as_ptr());  // Get the location of the uniform parameter in the fragment shader.
        program.apply();
        gl::Uniform1i(texture_uni_id, 0);
    }

    let scale: f32 = 1.5;
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
                    program.apply();  // Tell OpenGL to use this shader program.
                    gl::Uniform1f(uniform_id, scale);  // Set the uniform value.
                    texture.bind();
                    vertex_array.bind();  // Bind the VERTICES so OpenGL uses it.
                    gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());  // Draw the triangle using GL_TRIANGLES primitive.
                }

                // at the end of the full draw, swap the draw buffer with the current buffer
                gl_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

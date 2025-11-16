use std::{ffi::CString, path::Path, ptr, time::Instant};
use gl::types::{GLint, GLsizei, GLuint};
use glm::{Mat4, Vector3};
use glutin::{Api, ContextBuilder, GlRequest, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};
use opengl_test::{math::mat4::{Identity, ToColsArray}, set_attribute, shaders::{Buffer, Color, Pos, Pos3, Shader, ShaderProgram, Texture, VertexArray}};

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
        .with_vsync(true)
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let vertices: [Vertex; 5] = [
        Vertex([-0.5, 0.0, 0.5], [0.1, 0.2, 0.3], [0.0, 0.0]),
        Vertex([-0.5, 0.0, -0.5], [0.9, 0.2, 0.9], [5.0, 0.0]),
        Vertex([0.5, 0.0, -0.5], [0.4, 0.5, 0.3], [0.0, 0.0]),
        Vertex([0.5, 0.0, 0.5], [0.1, 0.2, 0.9], [5.0, 0.0]),
        Vertex([0.0, 0.8, 0.0], [0.1, 0.2, 0.9], [2.5, 5.0]),
    ];

    let indices: [GLuint; 18] = [
        0, 1, 2,
        0, 2, 3,
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
        3, 0, 4,
    ];

    let program;
    let _vertex_buffer;
    let _indeces_buffer;
    let vertex_array;
    let uniform_id: GLint;
    let texture;
    unsafe {
        // Create the two shaders.
        let vertex_shader = Shader::from_file("assets/shaders/5-third-dimension.vert", gl::VERTEX_SHADER).unwrap();
        let fragment_shader = Shader::from_file("assets/shaders/5-third-dimension.frag", gl::FRAGMENT_SHADER).unwrap();
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

        let _ = texture.load(&Path::new("assets/textures/bedrock.png")).unwrap();

        let location = CString::new("tex0").unwrap();
        let texture_uni_id = gl::GetUniformLocation(program.id, location.as_ptr());  // Get the location of the uniform parameter in the fragment shader.
        program.apply();
        gl::Uniform1i(texture_uni_id, 0);

        gl::Enable(gl::DEPTH_TEST);
    }

    let scale: f32 = 1.5;
    let mut rotation: f32 = 0.0;
    let mut prev_loop_time = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        if prev_loop_time.elapsed().as_secs_f32() >= 1f32 / 60f32 {
            rotation += 0.5;
            prev_loop_time = Instant::now();
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
                    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                    program.apply();  // Tell OpenGL to use this shader program.


                    let mut model = Mat4::one();
                    let mut view = Mat4::one();
                    let proj = glm::ext::perspective(glm::radians(45.0), 800f32 / 800f32, 0.1, 100.0);
                    model = glm::ext::rotate(&model, glm::radians(rotation), Vector3::new(0.0, 1.0, 0.0));
                    view = glm::ext::translate(&view, Vector3::new(0.0, -0.4, -2.0));


                    let loc_model = program.get_uniform_attrib_location("model").unwrap();
                    gl::UniformMatrix4fv(loc_model, 1, gl::FALSE, model.to_cols_array().as_ptr());
                    let loc_view = program.get_uniform_attrib_location("view").unwrap();
                    gl::UniformMatrix4fv(loc_view, 1, gl::FALSE, view.to_cols_array().as_ptr());
                    let loc_proj = program.get_uniform_attrib_location("proj").unwrap();
                    gl::UniformMatrix4fv(loc_proj, 1, gl::FALSE, proj.to_cols_array().as_ptr());


                    gl::Uniform1f(uniform_id, scale);  // Set the uniform value.
                    texture.bind();
                    vertex_array.bind();  // Bind the VERTICES so OpenGL uses it.
                    gl::DrawElements(gl::TRIANGLES, indices.len() as GLsizei, gl::UNSIGNED_INT, ptr::null());  // Draw the triangle using GL_TRIANGLES primitive.
                }

                // at the end of the full draw, swap the draw buffer with the current buffer
                gl_context.swap_buffers().unwrap();
            },
            _ => (),
        }
    });
}

use glfw::{Action, Context, Key};
use glow::HasContext;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::LOG_ERRORS).unwrap();

    // OpenGL Version
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    // Create window
    let (mut window, events) = glfw
        .create_window(800, 600, "Rust OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);

    // Load gl bindings
    let gl = unsafe {
        glow::Context::from_loader_function(|s| glfw.get_proc_address_raw(s) as *const _)
    };

    let (vbo, vao, ebo, program) = unsafe {
        // View setup
        gl.viewport(0, 0, 800, 600);

        // Wireframe
        gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        // Vertex Shader
        let vertex_shader = gl
            .create_shader(glow::VERTEX_SHADER)
            .expect("Failed to create shader");
        let vertex_source = include_str!("../res/vertex.glsl");

        gl.shader_source(vertex_shader, vertex_source);
        gl.compile_shader(vertex_shader);

        if !gl.get_shader_compile_status(vertex_shader) {
            panic!("{}", gl.get_shader_info_log(vertex_shader));
        }

        // Fragment Shader
        let fragment_shader = gl
            .create_shader(glow::FRAGMENT_SHADER)
            .expect("Failed to create shader");
        let fragment_source = include_str!("../res/fragment.glsl");

        gl.shader_source(fragment_shader, fragment_source);
        gl.compile_shader(fragment_shader);

        if !gl.get_shader_compile_status(fragment_shader) {
            panic!("{}", gl.get_shader_info_log(fragment_shader));
        }

        // Program
        let program = gl.create_program().expect("Failed to create program");
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);

        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        // Unlink and delete shaders
        gl.detach_shader(program, vertex_shader);
        gl.detach_shader(program, fragment_shader);

        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        // Triangle vertices
        #[rustfmt::skip]
        let vertices: [f32; 12] = [
             0.5,  0.5,  0.0,
             0.5, -0.5,  0.0,
            -0.5, -0.5,  0.0,
            -0.5,  0.5,  0.0,
        ];

        #[rustfmt::skip]
        let indicies: [u32; 6] = [
            0, 1, 3,
            1, 2, 3,
        ];

        // Vertex Array Object
        let vao = gl
            .create_vertex_array()
            .expect("Failed to create vertex array");

        gl.bind_vertex_array(Some(vao));

        // Vertex Buffer Object
        let vbo = gl.create_buffer().expect("Failed to create vertex buffer");
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );

        // Element Buffer Object
        let ebo = gl.create_buffer().expect("Failed to create element buffer");
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            indicies.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );

        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 3,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        (vbo, vao, ebo, program)
    };

    // Main Loop
    while !window.should_close() {
        // Render
        unsafe {
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }

        // Swap buffers and poll
        window.swap_buffers();
        glfw.poll_events();

        // Handle Events
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl.viewport(0, 0, width, height) };
                }
                _ => {}
            }
        }
    }

    // Cleanup
    unsafe {
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo);
        gl.delete_buffer(ebo);
        gl.delete_program(program);
    }
}

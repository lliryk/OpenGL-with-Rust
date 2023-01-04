use std::rc::Rc;

use glfw::{Action, Context, Key};
use glow::HasContext;
use learn_opengl::shader::Shader;

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
    let gl = Rc::new(unsafe {
        glow::Context::from_loader_function(|s| glfw.get_proc_address_raw(s) as *const _)
    });

    let (vbo, vao, ebo, shader) = unsafe {
        // View setup
        gl.viewport(0, 0, 800, 600);

        // Wireframe
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        // Shader
        let shader = Shader::from(
            Rc::clone(&gl),
            include_str!("../res/vertex.glsl"),
            include_str!("../res/fragment.glsl"),
        )
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        });

        // Triangle vertices
        #[rustfmt::skip]
        let vertices: [f32; 24] = [
             0.5,  0.5,  0.0, 1.0, 0.0, 0.0,
             0.5, -0.5,  0.0, 0.0, 1.0, 0.0,
            -0.5, -0.5,  0.0, 0.0, 0.0, 1.0,
            -0.5,  0.5,  0.0, 0.0, 0.0, 0.0,
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

        // aPos position attribute
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 6,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        // aPos position attribute
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 6,
            std::mem::size_of::<f32>() as i32 * 3,
        );
        gl.enable_vertex_attrib_array(1);

        (vbo, vao, ebo, shader)
    };

    // Main Loop
    while !window.should_close() {
        // Render
        unsafe {
            // Clear color
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            // Get vertex color
            // let time = glfw.get_time();
            // let green_value = (time.sin() / 2.0) + 0.5;
            // let vertex_color_location = gl
            //     .get_uniform_location(program, "ourColor")
            //     .expect("Failed to retrieve uniform");

            // Draw square
            shader.bind();
            // gl.uniform_4_f32(
            //     Some(&vertex_color_location),
            //     0.0,
            //     green_value as f32,
            //     0.0,
            //     0.0,
            // );
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
    }
}

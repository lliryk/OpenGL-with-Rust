use std::{io::BufReader, rc::Rc};

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

    // Load texture files
    // Container.jpg
    let image1_file =
        std::fs::File::open("res/container.jpg").expect("Could not open texture file");

    let image1_buffer = BufReader::new(image1_file);

    let image1 = image::load(image1_buffer, image::ImageFormat::Jpeg)
        .expect("Failed to process image")
        .flipv();

    // AwesomeFace.png
    let image2_file =
        std::fs::File::open("res/awesomeface.png").expect("Could not open texture file");

    let image2_buffer = BufReader::new(image2_file);

    let image2 = image::load(image2_buffer, image::ImageFormat::Png)
        .expect("Failed to process image")
        .flipv();

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
    let (vbo, vao, ebo, texture1, texture2) = unsafe {
        // View setup
        gl.viewport(0, 0, 800, 600);

        // Wireframe
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        // Triangle vertices
        #[rustfmt::skip]
        let vertices: [f32; 32] = [
            // Positions         // Colors         // Texture Coords
             0.5,  0.5,  0.0,    1.0, 0.0, 0.0,    1.0, 1.0,
             0.5, -0.5,  0.0,    0.0, 1.0, 0.0,    1.0, 0.0,
            -0.5, -0.5,  0.0,    0.0, 0.0, 1.0,    0.0, 0.0,
            -0.5,  0.5,  0.0,    1.0, 1.0, 0.0,    0.0, 1.0,
        ];

        #[rustfmt::skip]
        let indicies: [u32; 6] = [
            0, 1, 3,
            1, 2, 3,
        ];

        // Texture 1
        let texture1 = gl.create_texture().expect("Failed to create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture1));

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32, // Thanks OpenGL
            image1.width() as i32,
            image1.height() as i32,
            0,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            Some(image1.as_bytes()),
        );

        gl.generate_mipmap(glow::TEXTURE_2D);

        // Texture 2
        let texture2 = gl.create_texture().expect("Failed to create texture");
        gl.bind_texture(glow::TEXTURE_2D, Some(texture2));

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32, // Thanks OpenGL
            image2.width() as i32,
            image2.height() as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(image2.as_bytes()),
        );

        gl.generate_mipmap(glow::TEXTURE_2D);

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
            std::mem::size_of::<f32>() as i32 * 8,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        // aPos position attribute
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            std::mem::size_of::<f32>() as i32 * 3,
        );
        gl.enable_vertex_attrib_array(1);

        // aPos position attribute
        gl.vertex_attrib_pointer_f32(
            2,
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            std::mem::size_of::<f32>() as i32 * 6,
        );
        gl.enable_vertex_attrib_array(2);

        (vbo, vao, ebo, texture1, texture2)
    };

    // Bind texture uniforms
    shader.bind();
    shader.set_int("texture2", 1);

    // Main Loop
    while !window.should_close() {
        // Render
        unsafe {
            // Clear color
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);

            let identity = glam::Mat4::IDENTITY;

            let translation = glam::Mat4::from_translation(glam::vec3(0.5, -0.5, 0.0));
            let rotation = glam::Mat4::from_rotation_z(glfw.get_time() as f32);

            let transform = identity.mul_mat4(&translation.mul_mat4(&rotation));

            // Draw square
            shader.bind();

            shader.set_mat4_float("transform", false, &transform.to_cols_array());

            // Bind textures
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture1));
            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture2));

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

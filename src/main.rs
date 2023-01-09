use std::{io::BufReader, rc::Rc};

use glfw::{Action, Context, Key};
use glow::HasContext;
use learn_opengl::{
    camera::{Camera, Movement},
    shader::Shader,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

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
        .create_window(WIDTH, HEIGHT, "Rust OpenGL", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

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
    let (vbo, vao, texture1, texture2) = unsafe {
        // View setup
        gl.viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl.enable(glow::DEPTH_TEST);

        // Wireframe
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        // Triangle vertices
        #[rustfmt::skip]
        let vertices: [f32; 180] = [
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
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

        // aPos position attribute
        gl.vertex_attrib_pointer_f32(
            0,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        // aTexCoord position attribute
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 5,
            std::mem::size_of::<f32>() as i32 * 3,
        );
        gl.enable_vertex_attrib_array(1);

        (vbo, vao, texture1, texture2)
    };

    // Bind texture uniforms
    shader.bind();
    shader.set_int("texture1", 0);
    shader.set_int("texture2", 1);

    let cube_positions = [
        glam::vec3(0.0, 0.0, 0.0),
        glam::vec3(2.0, 5.0, -15.0),
        glam::vec3(-1.5, -2.2, -2.5),
        glam::vec3(-3.8, -2.0, -12.3),
        glam::vec3(2.4, -0.4, -3.5),
        glam::vec3(-1.7, 3.0, -7.5),
        glam::vec3(1.3, -2.0, -2.5),
        glam::vec3(1.5, 2.0, -2.5),
        glam::vec3(1.5, 0.2, -1.5),
        glam::vec3(-1.3, 1.0, -1.5),
    ];

    // Frame Timing
    let mut delta_time;
    let mut last_frame = 0.0f32;

    // Camera
    let mut camera = Camera::default();
    let mut first_mouse = false;
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    // Main Loop
    while !window.should_close() {
        let cur_frame = glfw.get_time() as f32;
        delta_time = cur_frame - last_frame;
        last_frame = cur_frame;

        // Render
        unsafe {
            // Clear color
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // Shader
            shader.bind();

            let view = camera.get_viewmatrix();
            let projection = glam::Mat4::perspective_rh_gl(
                camera.get_fov().to_radians(),
                WIDTH as f32 / HEIGHT as f32,
                0.1,
                100.0,
            );

            shader.set_mat4("projection", false, &projection);
            shader.set_mat4("view", false, &view);

            // Bind textures
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture1));
            gl.active_texture(glow::TEXTURE1);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture2));

            // Draw model
            gl.bind_vertex_array(Some(vao));
            for (i, pos) in cube_positions.iter().enumerate() {
                let mut model = glam::Mat4::IDENTITY * glam::Mat4::from_translation(*pos);
                let angle = i * 20;
                model *= glam::Mat4::from_axis_angle(
                    glam::vec3(1.0, 0.3, 0.5).normalize(),
                    (angle as f32).to_radians(),
                );
                shader.set_mat4("model", false, &model);
                gl.draw_arrays(glow::TRIANGLES, 0, 36);
            }
        }

        // Swap buffers and poll
        window.swap_buffers();
        glfw.poll_events();

        // Handle Events
        for (_, event) in glfw::flush_messages(&events) {
            eprintln!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => match key {
                    Key::Escape if action == Action::Press => window.set_should_close(true),
                    Key::W if action != Action::Release => {
                        camera.move_position(Movement::Forward, delta_time)
                    }
                    Key::S if action != Action::Release => {
                        camera.move_position(Movement::BackWard, delta_time)
                    }
                    Key::A if action != Action::Release => {
                        camera.move_position(Movement::Left, delta_time)
                    }
                    Key::D if action != Action::Release => {
                        camera.move_position(Movement::Right, delta_time)
                    }
                    _ => {}
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    if first_mouse {
                        last_x = x as f32;
                        last_y = y as f32;
                        first_mouse = false;
                    }
                    let x_offset = x as f32 - last_x;
                    let y_offset = last_y - y as f32;

                    last_x = x as f32;
                    last_y = y as f32;

                    camera.move_view(x_offset, y_offset);
                }
                glfw::WindowEvent::Scroll(_, y) => camera.change_zoom(y as f32),
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
        // gl.delete_buffer(ebo);
    }
}

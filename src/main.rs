use std::{path::Path, rc::Rc};

use glfw::{Action, Context, Key};
use glow::HasContext;
use learn_opengl::{
    camera::{Camera, Movement},
    shader::{self, Shader},
    texture::Texture,
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

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    // Load gl bindings
    let gl = Rc::new(unsafe {
        glow::Context::from_loader_function(|s| glfw.get_proc_address_raw(s) as *const _)
    });

    // Shaders
    let mut lighting_shader = Shader::from_str(
        Rc::clone(&gl),
        include_str!("../res/shaders/color.vert"),
        include_str!("../res/shaders/color.frag"),
    )
    .unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    {
        let diffuse_texture = Texture::new(Rc::clone(&gl), Path::new("res/container2.png"))
            .unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
        lighting_shader
            .add_texture(diffuse_texture, shader::TextureIndex::Index0)
            .expect("Texture index should not be occupied");

        let specular_texture =
            Texture::new(Rc::clone(&gl), Path::new("res/container2_specular.png")).unwrap_or_else(
                |e| {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                },
            );
        lighting_shader
            .add_texture(specular_texture, shader::TextureIndex::Index1)
            .expect("Texture index should not be occupied");
    }

    let light_cube_shader = Shader::from_str(
        Rc::clone(&gl),
        include_str!("../res/shaders/light_cube.vert"),
        include_str!("../res/shaders/light_cube.frag"),
    )
    .unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let (vbo, cube_vao, light_vao) = unsafe {
        // View setup
        gl.viewport(0, 0, WIDTH as i32, HEIGHT as i32);
        gl.enable(glow::DEPTH_TEST);

        // Wireframe
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        // Triangle vertices
        #[rustfmt::skip]
        let vertices: [f32; 288] = [
            // positions          // normals           // texture coords
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
             0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
             0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,
             0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 0.0,
             0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
             0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,

            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
            -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
            -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
            -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
            -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
             0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
             0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
             0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
             0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
        ];

        // Vertex Array Object
        let cube_vao = gl
            .create_vertex_array()
            .expect("Failed to create vertex array");

        gl.bind_vertex_array(Some(cube_vao));

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
            std::mem::size_of::<f32>() as i32 * 8,
            0,
        );
        gl.enable_vertex_attrib_array(0);

        // aNormal
        gl.vertex_attrib_pointer_f32(
            1,
            3,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            std::mem::size_of::<f32>() as i32 * 3,
        );
        gl.enable_vertex_attrib_array(1);

        // aNormal
        gl.vertex_attrib_pointer_f32(
            2,
            2,
            glow::FLOAT,
            false,
            std::mem::size_of::<f32>() as i32 * 8,
            std::mem::size_of::<f32>() as i32 * 6,
        );
        gl.enable_vertex_attrib_array(2);

        let light_vao = gl
            .create_vertex_array()
            .expect("Failed to create vertex array");

        gl.bind_vertex_array(Some(light_vao));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

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

        (vbo, cube_vao, light_vao)
    };

    // Frame Timing
    let mut delta_time;
    let mut last_frame = 0.0f32;

    // Camera
    let mut camera = Camera::default();
    let mut first_mouse = false;
    let mut last_x = 0.0;
    let mut last_y = 0.0;

    // Light position
    let light_pos = glam::vec3(1.2, 1.0, 2.0);

    // Main Loop
    while !window.should_close() {
        let cur_frame = glfw.get_time() as f32;
        delta_time = cur_frame - last_frame;
        last_frame = cur_frame;

        // Render
        unsafe {
            // Clear color
            gl.clear_color(0.1, 0.1, 0.1, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // Shader
            lighting_shader.bind();

            // Color / lighting
            lighting_shader.set_int("material.diffuse", 0);
            lighting_shader.set_int("material.specular", 1);
            lighting_shader.set_float("material.shininess", 32.0);

            lighting_shader.set_vec3("light.position", light_pos);

            let light_color = glam::vec3(
                (glfw.get_time() as f32 * 2.0).sin(),
                (glfw.get_time() as f32 * 0.7).sin(),
                (glfw.get_time() as f32 * 1.3).sin(),
            );

            let diffuse_color = light_color * glam::Vec3::splat(0.5);
            let ambient_color = light_color * glam::Vec3::splat(0.2);

            lighting_shader.set_vec3("light.ambient", ambient_color);
            lighting_shader.set_vec3("light.diffuse", diffuse_color);
            lighting_shader.set_vec3("light.specular", glam::Vec3::ONE);
            lighting_shader.set_vec3("viewPos", camera.position());

            // View / Projection
            let view = camera.get_viewmatrix();
            let projection = glam::Mat4::perspective_rh_gl(
                camera.fov().to_radians(),
                WIDTH as f32 / HEIGHT as f32,
                0.1,
                100.0,
            );

            lighting_shader.set_mat4("projection", false, &projection);
            lighting_shader.set_mat4("view", false, &view);

            // World transformations
            let model = glam::Mat4::IDENTITY;
            lighting_shader.set_mat4("model", false, &model);

            // Draw cube model
            gl.bind_vertex_array(Some(cube_vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 36);

            // Draw light model
            light_cube_shader.bind();

            light_cube_shader.set_vec3("lightColor", light_color);

            light_cube_shader.set_mat4("projection", false, &projection);
            light_cube_shader.set_mat4("view", false, &view);

            let mut light_cube_model = glam::Mat4::IDENTITY;
            light_cube_model *= glam::Mat4::from_translation(light_pos);
            light_cube_model *= glam::Mat4::from_scale(glam::Vec3::splat(0.2));

            light_cube_shader.set_mat4("model", false, &light_cube_model);

            gl.bind_vertex_array(Some(light_vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 36);
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
        gl.delete_vertex_array(cube_vao);
        gl.delete_buffer(vbo);
        // gl.delete_buffer(ebo);
    }
}

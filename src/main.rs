extern crate gl;
extern crate image;
extern crate nalgebra_glm as glm;
extern crate sdl2;
mod controls;
mod load_bmp;
pub mod render_gl;
use std::time::Duration;
use std::time::Instant;

// use libc::c_void;
use std::ffi::CString;

use std::ffi::c_void;

#[allow(non_snake_case)]

// fn print_mat4(item: glm::Mat4) {
//     for r in 0..4 {
//         for c in 0..4 {
//             println!("{:?}", item[(r, c)]);
//         }
//     }
// }

// fn print_vec4(item: glm::Vec4) {
//     for r in 0..4 {
//         println!("{:?}", item[r]);
//     }
// }

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // let texture_img = load_bmp::load();
    let texture = load_bmp::Bitmap::open(&String::from("resources/wall.bmp")).unwrap();
    texture.print();
    // let mut horizontalAngle = 0.0;
    // let mut verticalAngle = 0.0;
    // let mouseSpeed = 0.1;

    // position
    let position = glm::vec3(0, 0, 5);
    // horizontal angle : toward -Z
    let mut horizontalAngle = 3.14;
    // vertical angle : 0, look at the horizon
    let mut verticalAngle = 0.0;
    // Initial Field of View
    let mut initialFoV = 45.0;

    let speed = 3.0; // 3 units / second
    let mouseSpeed = 0.0005;
    let mut prevMouseX = 0;
    let mut prevMouseY = 0;
    let now = Instant::now();
    let mut elapsed = now.elapsed();
    let mut lastTime = elapsed.as_millis();

    //event
    let mut event_pump = sdl.event_pump().unwrap();

    //shader
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();
    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();
    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    let vertices: Vec<f32> = vec![
        -0.5, -0.0, 0.0, 0.5, -0.0, 0.0, 0.0, 0.5, 0.0, -0.5, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, -0.5,
        0.0,
    ];

    let tri_vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    let cube_vertices: Vec<f32> = vec![
        -1.0, -1.0, -1.0, // triangle 1 : begin
        -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, // triangle 1 : end
        1.0, 1.0, -1.0, // triangle 2 : begin
        -1.0, -1.0, -1.0, -1.0, 1.0, -1.0, // triangle 2 : end
        1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, -1.0,
        -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, -1.0,
        1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, -1.0, 1.0, 1.0, 1.0, -1.0, 1.0,
    ];

    let cube_colors: Vec<f32> = vec![
        0.583, 0.771, 0.014, 0.609, 0.115, 0.436, 0.327, 0.483, 0.844, 0.822, 0.569, 0.201, 0.435,
        0.602, 0.223, 0.310, 0.747, 0.185, 0.597, 0.770, 0.761, 0.559, 0.436, 0.730, 0.359, 0.583,
        0.152, 0.483, 0.596, 0.789, 0.559, 0.861, 0.639, 0.195, 0.548, 0.859, 0.014, 0.184, 0.576,
        0.771, 0.328, 0.970, 0.406, 0.615, 0.116, 0.676, 0.977, 0.133, 0.971, 0.572, 0.833, 0.140,
        0.616, 0.489, 0.997, 0.513, 0.064, 0.945, 0.719, 0.592, 0.543, 0.021, 0.978, 0.279, 0.317,
        0.505, 0.167, 0.620, 0.077, 0.347, 0.857, 0.137, 0.055, 0.953, 0.042, 0.714, 0.505, 0.345,
        0.783, 0.290, 0.734, 0.722, 0.645, 0.174, 0.302, 0.455, 0.848, 0.225, 0.587, 0.040, 0.517,
        0.713, 0.338, 0.053, 0.959, 0.120, 0.393, 0.621, 0.362, 0.673, 0.211, 0.457, 0.820, 0.883,
        0.371, 0.982, 0.099, 0.879,
    ];

    let cube_textures: Vec<f32> = vec![
        0.000059,
        1.0 - 0.000004,
        0.000103,
        1.0 - 0.336048,
        0.335973,
        1.0 - 0.335903,
        1.000023,
        1.0 - 0.000013,
        0.667979,
        1.0 - 0.335851,
        0.999958,
        1.0 - 0.336064,
        0.667979,
        1.0 - 0.335851,
        0.336024,
        1.0 - 0.671877,
        0.667969,
        1.0 - 0.671889,
        1.000023,
        1.0 - 0.000013,
        0.668104,
        1.0 - 0.000013,
        0.667979,
        1.0 - 0.335851,
        0.000059,
        1.0 - 0.000004,
        0.335973,
        1.0 - 0.335903,
        0.336098,
        1.0 - 0.000071,
        0.667979,
        1.0 - 0.335851,
        0.335973,
        1.0 - 0.335903,
        0.336024,
        1.0 - 0.671877,
        1.000004,
        1.0 - 0.671847,
        0.999958,
        1.0 - 0.336064,
        0.667979,
        1.0 - 0.335851,
        0.668104,
        1.0 - 0.000013,
        0.335973,
        1.0 - 0.335903,
        0.667979,
        1.0 - 0.335851,
        0.335973,
        1.0 - 0.335903,
        0.668104,
        1.0 - 0.000013,
        0.336098,
        1.0 - 0.000071,
        0.000103,
        1.0 - 0.336048,
        0.000004,
        1.0 - 0.671870,
        0.336024,
        1.0 - 0.671877,
        0.000103,
        1.0 - 0.336048,
        0.336024,
        1.0 - 0.671877,
        0.335973,
        1.0 - 0.335903,
        0.667969,
        1.0 - 0.671889,
        1.000004,
        1.0 - 0.671847,
        0.667979,
        1.0 - 0.335851,
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (cube_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            cube_vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                                    // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    //cbuf
    let mut cbuf: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut cbuf);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, cbuf);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                          // target
            (cube_colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            cube_colors.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                                  // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }
    //cbuf end

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        //color
        gl::BindBuffer(gl::ARRAY_BUFFER, cbuf);
        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            1,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        //end

        //texture
        gl::BindBuffer(gl::ARRAY_BUFFER, cbuf);
        gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
        gl::VertexAttribPointer(
            2,         // index of the generic vertex attribute ("layout (location = 2)")
            2,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (2 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        //end
    }

    'main: loop {
        //event handler
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // controls::test1();
        // Compute the MVP matrix from keyboard and mouse input
        // computeMatricesFromInputs();
        // let ProjectionMatrix = getProjectionMatrix();
        // let ViewMatrix = getViewMatrix();
        // let ModelMatrix = glm::identity(); //needs check
        // let MVP = ProjectionMatrix * ViewMatrix * ModelMatrix;

        //window
        let (mut width, height) = window.size();
        // println!("width{}", width);
        // println!("height{}", height);

        //mouse
        let mut mouse = event_pump.mouse_state();
        // println!("x{}", mouse.x());
        // println!("y{}", mouse.y());

        // let now = Instant::now();
        elapsed = now.elapsed();
        // let mut lastTime = elapsed.as_millis();
        let currentTime = elapsed.as_millis();
        let deltaTime = (currentTime - lastTime) as f32;
        // println!("delta{}", deltaTime);

        lastTime = currentTime;
        horizontalAngle += mouseSpeed * deltaTime * (prevMouseX - mouse.x()) as f32;
        verticalAngle += mouseSpeed * deltaTime * (prevMouseY - mouse.y()) as f32;
        prevMouseX = mouse.x();
        prevMouseY = mouse.y();
        // horizontalAngle = mouse.x() as f32;
        // verticalAngle = mouse.y() as f32;
        let direction = glm::vec3(
            f32::cos(verticalAngle) * f32::sin(horizontalAngle),
            f32::sin(verticalAngle),
            f32::cos(verticalAngle) * f32::cos(horizontalAngle),
        );

        //manipulate trans
        let vec = glm::vec4(0.0, 0.0, 0.0, 1.0);
        let trans = glm::identity();
        let trans = glm::translate(&trans, &glm::vec3(-0.5, 0.5, 0.0)); //translate
        let trans = glm::rotate(
            &trans,
            // glm::radians(&glm::vec1(0.0))[0],
            (currentTime as f32) / 1000.0,
            &glm::vec3(0.0, 1.0, 0.0),
        );
        let trans = glm::scale(&trans, &glm::vec3(1.0, 1.0, 1.0)); //scale

        let Projection = glm::perspective(
            glm::radians(&glm::vec1(45.0))[0],
            width as f32 / height as f32,
            0.1,
            100.0,
        );
        let cam_pos = glm::vec3(4.0, 3.0, 3.0);
        let origin = glm::vec3(0.0, 0.0, 0.0);
        let View = glm::look_at(
            // &glm::vec3(4.0, 3.0, 3.0), // Camera is at (4,3,3), in World Space
            &cam_pos,
            // &(cam_pos + direction), // and looks at the origin
            &origin,
            &glm::vec3(0.0, 1.0, 0.0), // Head is up (set to 0,-1,0 to look upside-down)
        );
        let Model = trans;
        let mvp = Projection * View * Model;

        shader_program.set_used();

        // pass uniform to shader
        unsafe {
            let c_str = CString::new("mvp").unwrap();
            let uniformLoc = gl::GetUniformLocation(shader_program.id(), c_str.as_ptr());
            gl::UniformMatrix4fv(uniformLoc, 1, gl::FALSE, glm::value_ptr(&mvp).as_ptr());
        }

        // Create one OpenGL texture
        unsafe {
            let mut textureID = 0;
            gl::GenTextures(1, &mut textureID);

            // "Bind" the newly created texture : all future texture functions will modify this texture
            gl::BindTexture(gl::TEXTURE_2D, textureID);

            let mut data = [(100, 0, 0); 512 * 512];
            // Give the image to OpenGL
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                512,
                512,
                0,
                gl::BGR,
                gl::UNSIGNED_BYTE,
                &data[0] as *const _ as *const c_void,
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }

        // render triangles
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3 * 12,        // number of indices to be rendered
            );
        }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

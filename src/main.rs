extern crate gl;
extern crate nalgebra_glm as glm;
extern crate sdl2;
pub mod render_gl;

// use libc::c_void;
use std::ffi::CString;

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

    //event
    let mut event_pump = sdl.event_pump().unwrap();
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

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    //cbuf
    // let mut cbuf: gl::types::GLuint = 0;
    // unsafe {
    //     gl::GenBuffers(1, &mut cbuf);
    // }

    // unsafe {
    //     gl::BindBuffer(gl::ARRAY_BUFFER, cbuf);
    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,                                                       // target
    //         (cube_colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
    //         cube_colors.as_ptr() as *const gl::types::GLvoid, // pointer to data
    //         gl::STATIC_DRAW,                               // usage
    //     );
    //     gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    // }
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
        // gl::BindBuffer(gl::ARRAY_BUFFER, cbuf);
        // gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
        // gl::VertexAttribPointer(
        //     1,         // index of the generic vertex attribute ("layout (location = 0)")
        //     3,         // the number of components per generic vertex attribute
        //     gl::FLOAT, // data type
        //     gl::FALSE, // normalized (int-to-float conversion)
        //     (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        //     std::ptr::null(), // offset of the first component
        // );

        // gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // gl::BindVertexArray(0);
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
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        //manipulate trans
        let vec = glm::vec4(0.0, 0.0, 0.0, 1.0);
        let trans = glm::identity();
        let trans = glm::translate(&trans, &glm::vec3(-0.5, 0.5, 0.0)); //translate
        let trans = glm::rotate(
            &trans,
            glm::radians(&glm::vec1(45.0))[0],
            &glm::vec3(0.0, 0.0, 1.0),
        );
        let trans = glm::scale(&trans, &glm::vec3(0.5, 0.5, 0.5)); //scale

        shader_program.set_used();

        // pass uniform to shader
        unsafe {
            let c_str = CString::new("trans").unwrap();
            let uniformLoc = gl::GetUniformLocation(shader_program.id(), c_str.as_ptr());
            gl::UniformMatrix4fv(uniformLoc, 1, gl::FALSE, glm::value_ptr(&trans).as_ptr());
        }

        // render triangles
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3 * 2,         // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}

extern crate gl;
extern crate image;
extern crate nalgebra_glm as glm;
extern crate sdl2;
extern crate num;

pub mod buffer;
mod parse;
mod mvp;
pub mod render_gl;
pub mod matrix;
pub mod vector;
mod texture;
pub mod camera;
mod macros;
pub mod model;

// use crate::vector::{Vector, TVector3};
use matrix::Matrix;
use buffer::Buffer;
use mvp::get_mvp;
// use render_gl::{Shader, Program};
pub mod shader;
use shader::Shader;
use vector::TVector3;
use std::ffi::{CString, CStr};
use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use num::{Float};
use camera::Camera;
use model::Model;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use self::gl::types::*;

const SCR_WIDTH: u32 = 600;
const SCR_HEIGHT: u32 = 600;

fn load_buf() -> (Vec<f32>, gl::types::GLuint) {
    let vertices = parse::parse("resources/obj/teapot.obj");
    let vertex_buf = Buffer::new();
    vertex_buf.bind(&vertices);

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        vertex_buf.enable();
    }
    let colors: [f32; 9] = [
        1., 0.5, 0.0, // left
        0.5, 0.5, 0.0, // right
        0.5,  0.5, 0.0  // top
    ];
    let color_buf = Buffer::new();
    color_buf.bind(&Vec::from(colors));
    color_buf.enable();
    let textures: [f32; 12] = [
        1.0, 1.0,
        1.0, 0.0,
        0.0, 1.0,
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0
    ];
    // let text_buf = Buffer::new();
    // text_buf.bind(&Vec::from(textures));
    // text_buf.enable_texture();
    // texture::texture();

    unsafe {
        gl::BindVertexArray(0); // Call this when all the bindings are done
    }
    (vertices, vao)
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", SCR_WIDTH, SCR_HEIGHT)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    //event
    let mut event_pump = sdl.event_pump().unwrap();





//==============================
let (lightingShader, lampShader, VBO, cubeVAO, lightVAO) = unsafe {
    // configure global opengl state
    // -----------------------------
    gl::Enable(gl::DEPTH_TEST);

    // build and compile our shader program
    // ------------------------------------
    let lightingShader = Shader::new(
        "src/shaders/2.1.basic_lighting.vs",
        "src/shaders/2.1.basic_lighting.fs");
    let lampShader = Shader::new(
        "src/shaders/2.1.lamp.vs",
        "src/shaders/2.1.lamp.fs");

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    let vertices: [f32; 216] = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,

        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5, -0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
         0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0,  1.0,
        -0.5, -0.5,  0.5,  0.0,  0.0,  1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,
        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,
         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,
        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,
        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0
    ];
    // first, configure the cube's VAO (and VBO)
    let (mut VBO, mut cubeVAO) = (0, 0);
    gl::GenVertexArrays(1, &mut cubeVAO);
    gl::GenBuffers(1, &mut VBO);

    gl::BindVertexArray(cubeVAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(gl::ARRAY_BUFFER,
                   (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                   &vertices[0] as *const f32 as *const c_void,
                   gl::STATIC_DRAW);

    let stride = 6 * mem::size_of::<GLfloat>() as GLsizei;
    // position attribute
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);
    // normal attribute
    gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
    gl::EnableVertexAttribArray(1);


    // second, configure the light's VAO (VBO stays the same; the vertices are the same for the light object which is also a 3D cube)
    let mut lightVAO = 0;
    gl::GenVertexArrays(1, &mut lightVAO);
    gl::BindVertexArray(lightVAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);

    // note that we update the lamp's position attribute's stride to reflect the updated buffer data
    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    gl::EnableVertexAttribArray(0);

    (lightingShader, lampShader, VBO, cubeVAO, lightVAO)
};
//==============================













    // //shader
    // let vert_shader =
    //     Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
    //         .unwrap();
    // let frag_shader =
    //     Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
    //         .unwrap();
    // let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    // shader_program.set_used();
    // // let (vertices, vao) = load_buf();
    // let mut models: Vec<Model> = Vec::new();
    // models.push(Model::new("resources/obj/cube.obj"));
    // models.push(Model::new("resources/obj/teapot.obj"));
    // models.push(Model::new("resources/obj/teapot.obj"));


    let mut camera = Camera::new(
        TVector3::from([0., 0., 10.]),
        TVector3::from([0., 0., 0.]),
        TVector3::from([0., 1., 0.]),
    );
    while process_events(&mut event_pump, &mut camera) {


//==============================
// let lightPos = vec!(1.2, 1.0, 2.0);

unsafe {
    gl::ClearColor(0.2, 0.3, 0.3, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    // be sure to activate shader when setting uniforms/drawing objects
    lightingShader.useProgram();
    lightingShader.setVec3(c_str!("objectColor"), 1.0, 0.5, 0.31);
    lightingShader.setVec3(c_str!("lightColor"), 1.0, 1.0, 1.0);
    lightingShader.setVec3(c_str!("lightPos"), 1.2, 1.0, 2.0);

    // view/projection transformations
    let projection = mvp::projection();
    let view = camera.look_at();
    lightingShader.setMat4(c_str!("projection"), &projection);
    lightingShader.setMat4(c_str!("view"), &view);

    // world transformation
    let mut model = Matrix::identity();
    lightingShader.setMat4(c_str!("model"), &model);

    // render the cube
    gl::BindVertexArray(cubeVAO);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);


    // also draw the lamp object
    lampShader.useProgram();
    lampShader.setMat4(c_str!("projection"), &projection);
    lampShader.setMat4(c_str!("view"), &view);
    model = Matrix::translation(1.2, 1.0, 2.0);
    model = model * Matrix::scale(0.2, 0.2, 0.2);  // a smaller cube
    lampShader.setMat4(c_str!("model"), &model);

    gl::BindVertexArray(lightVAO);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}








//===============================











        // let (w, h) = window.size();
        // unsafe {
        //     gl::Viewport(0, 0, w as i32, h as i32);
        //     gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        //     gl::Enable(gl::DEPTH_TEST);
        //     gl::DepthFunc(gl::LESS);
        //     gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        // }
        // shader_program.set_used();
        // unsafe {
        //     shader_program.setMat4(c_str!("view"), &camera.look_at());
        //     shader_program.setMat4(c_str!("projection"), &mvp::projection());    
        // }


        // for (i, m) in models.iter_mut().enumerate() {
        //     // pass uniform to shader
        //     // let mvp = get_mvp(&mut camera);

        //     unsafe {
        //         println!("i{}", i);
        //         m.set_trans(i as f32, 0., 0.);
        //         println!("{}{:?}", i, m.get_model());
        //         shader_program.setMat4(c_str!("model"), &m.get_model());
        //         // shader_program.setMat4(c_str!("mvp"), &mvp);
        //         println!("vao{}", m.get_vao());
        //         gl::BindVertexArray(m.get_vao());
        //         gl::DrawArrays(gl::TRIANGLES,0, (m.get_vertices().len() / 3) as i32);
        //     }
        // }


        // // pass uniform to shader
        // let mvp = get_mvp(&mut camera);

        // unsafe {
        //     shader_program.setMat4(c_str!("mvp"), &mvp);
        //     gl::BindVertexArray(vao);
        //     gl::DrawArrays(gl::TRIANGLES,0, (vertices.len() / 3) as i32);
        // }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn process_events(event_pump: &mut sdl2::EventPump, camera: &mut Camera) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return false
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                camera.move_forward(0.1);

            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                camera.move_right(-0.1);

            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                camera.move_forward(-0.1);

            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                camera.move_right(0.1);

            },
            _ => {}
        }
    }
    return true
}

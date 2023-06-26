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

// use crate::vector::{Vector, TVector3};
use matrix::Matrix;
use buffer::Buffer;
use mvp::get_mvp;
use render_gl::{Shader, Program};
use vector::TVector3;
use std::ffi::{CString, CStr};
use std::time::Duration;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;
use num::{Float};
use camera::Camera;


const SCR_WIDTH: u32 = 600;
const SCR_HEIGHT: u32 = 600;

fn load_buf() -> (Vec<f32>, gl::types::GLuint) {
    let vertices = parse::parse("resources/teapot.obj");
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

    //shader
    let vert_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();
    let frag_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();
    let (vertices, vao) = load_buf();

    let mut camera = Camera::new(
        TVector3::from([0., 0., 10.]),
        TVector3::from([0., 0., 0.]),
        TVector3::from([0., 1., 0.]),
    );
    while process_events(&mut event_pump, &mut camera) {

        let (w, h) = window.size();
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        shader_program.set_used();

        // pass uniform to shader
        let mvp = get_mvp(&mut camera);

        unsafe {
            shader_program.setMat4(c_str!("mvp"), &mvp);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES,0, (vertices.len() / 3) as i32);
            // let c_str = CString::new("mvp").unwrap();
            // let uniform_loc = gl::GetUniformLocation(shader_program.id(), c_str.as_ptr());
            // gl::UniformMatrix4fv(uniform_loc, 1, gl::TRUE, mvp.as_mut_arr().as_mut_ptr() as * const f32);
        }

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

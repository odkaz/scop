extern crate gl;
extern crate sdl2;

pub mod buffer;
pub mod camera;
pub mod load_bmp;
mod macros;
pub mod matrix;
pub mod model;
mod mvp;
mod parse;
pub mod render_gl;
pub mod texture;
pub mod vector;

use camera::Camera;
use model::Model;
use render_gl::{Program, Shader};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use std::ffi::{CStr, CString};
use std::time::Duration;
use vector::TVector3;

const SCR_WIDTH: u32 = 1200;
const SCR_HEIGHT: u32 = 900;

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

    // //shader
    let vert_shader =
        Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap()).unwrap();
    let frag_shader =
        Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap()).unwrap();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();
    // let (vertices, vao) = load_buf();
    let mut models: Vec<Model> = Vec::new();
    models.push(Model::new("resources/obj/Airplane.obj"));
    // models.push(Model::new("resources/genshin_obj/barb.obj"));

    let mut camera = Camera::new(
        TVector3::from([0., 0., 10.]),
        TVector3::from([0., 0., 0.]),
        TVector3::from([0., 1., 0.]),
    );
    while process_events(&mut event_pump, &mut camera, &mut models) {
        let (w, h) = window.size();
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        shader_program.set_used();
        unsafe {
            shader_program.set_mat4(c_str!("view"), &camera.look_at());
            shader_program.set_mat4(
                c_str!("projection"),
                &mvp::projection(w as f32, h as f32, 135.),
            );
            shader_program.set_vec3(c_str!("objectColor"), 1.0, 0.5, 0.31);
            shader_program.set_vec3(c_str!("lightColor"), 1.0, 1.0, 1.0);
        }

        for (i, m) in models.iter_mut().enumerate() {
            unsafe {
                // m.set_trans(i as f32, i as f32, i as f32);
                // m.set_rot(0., mvp::timer(), 0.);
                // m.set_rot(0.0, 180.0, 0.0);
                // m.set_scale(0.2, 0.2, 0.2);
                shader_program.set_mat4(c_str!("model"), &m.get_model());
                gl::BindVertexArray(m.get_vao());
                gl::DrawArrays(gl::TRIANGLES, 0, (m.get_vertices().len() / 3) as i32);
            }
        }

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn is_pressed(event_pump: &mut sdl2::EventPump, code: Scancode) -> bool {
    event_pump.keyboard_state().is_scancode_pressed(code)
}

fn process_events(event_pump: &mut sdl2::EventPump, camera: &mut Camera, models: &mut Vec<Model>) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {}
        }
    }
    const VEL: f32 = 0.1;
    if is_pressed(event_pump, Scancode::W) {
        camera.move_forward(VEL);
    }
    if is_pressed(event_pump, Scancode::A) {
        camera.move_right(-VEL);
    }
    if is_pressed(event_pump, Scancode::S) {
        camera.move_forward(-VEL);
    }
    if is_pressed(event_pump, Scancode::D) {
        camera.move_right(VEL);
    }
    if is_pressed(event_pump, Scancode::E) {
        camera.move_up(VEL);
    }
    if is_pressed(event_pump, Scancode::Q) {
        camera.move_up(-VEL);
    }
    if is_pressed(event_pump, Scancode::Right) {
        models[0].move_x(VEL);
    }
    if is_pressed(event_pump, Scancode::Left) {
        models[0].move_x(-VEL);
    }
    if is_pressed(event_pump, Scancode::Up) {
        models[0].move_z(VEL);
    }
    if is_pressed(event_pump, Scancode::Down) {
        models[0].move_z(-VEL);
    }


    return true;
}

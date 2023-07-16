extern crate gl;
extern crate sdl2;

pub mod buffer;
pub mod camera;
pub mod load_bmp;
mod macros;
pub mod matrix;
pub mod model;
// pub mod model;
mod mvp;
mod parse;
// mod new_parse;
pub mod render_gl;
pub mod texture;
pub mod vector;

use camera::Camera;
// use model::Model;
use model::{Model, ModelGroup};
use render_gl::{Program, Shader};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use std::ffi::{CStr, CString};
use vector::TVector3;
use std::time::{Duration, SystemTime};

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

    let now = SystemTime::now();
    let mut models: ModelGroup = parse::parse("resources/obj/barbara.obj");
    models.texture_on(&shader_program);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("time: {}ms", elapsed.as_millis());
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }

    let mut camera = Camera::new(
        TVector3::from([0., 0., 10.]),
        TVector3::from([0., 0., 0.]),
        TVector3::from([0., 1., 0.]),
    );

    let text_id = models.init_textures(&shader_program);

    // let tex1 = texture::texture(&String::from("resources/textures/barbara/face.bmp"));
    // let tex2 = texture::texture(&String::from("resources/textures/barbara/body.bmp"));
    // unsafe {
    //     shader_program.set_float(c_str!("tex1Intensity"), 0.5);
    //     shader_program.set_float(c_str!("tex2Intensity"), 0.5);

    //     let tex1_loc  = gl::GetUniformLocation( shader_program.id(), c_str!("texture1").as_ptr());
    //     let tex2_loc  = gl::GetUniformLocation( shader_program.id(), c_str!("texture2").as_ptr());
    //     gl::Uniform1i(tex1_loc, 0);
    //     gl::Uniform1i(tex2_loc, 1);
    // }

    while process_events(&mut event_pump, &mut camera, &mut models, &shader_program) {
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
            shader_program.set_vec3(c_str!("objectColor"), 1.0, 1.0, 1.0);
            shader_program.set_vec3(c_str!("lightColor"), 1.0, 1.0, 1.0);
        }
        models.rotate(0.0, mvp::timer(), 0.0);
        // models.scale(0.1, 0.1, 0.1);


        unsafe {
            for (i, tid) in text_id.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, tid.clone());
            }
            // gl::ActiveTexture(gl::TEXTURE0);
            // gl::BindTexture(gl::TEXTURE_2D, tex1);
            // gl::ActiveTexture(gl::TEXTURE0 + 1);
            // gl::BindTexture(gl::TEXTURE_2D, tex2);
        }
        
        
        models.display(&shader_program);

        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn is_pressed(event_pump: &mut sdl2::EventPump, code: Scancode) -> bool {
    event_pump.keyboard_state().is_scancode_pressed(code)
}

fn process_events(event_pump: &mut sdl2::EventPump, camera: &mut Camera, models: &mut ModelGroup, shader_program: &Program) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            sdl2::event::Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            Event::KeyDown {
                keycode: Some(Keycode::T),
                ..
             } => {
                models.invert_texture(shader_program);
             },
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
        models.move_x(VEL);
    }
    if is_pressed(event_pump, Scancode::Left) {
        models.move_x(-VEL);
    }
    if is_pressed(event_pump, Scancode::Up) {
        models.move_z(VEL);
    }
    if is_pressed(event_pump, Scancode::Down) {
        models.move_z(-VEL);
    }


    return true;
}

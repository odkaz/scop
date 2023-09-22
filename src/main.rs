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
use model::ModelGroup;
use render_gl::{Program, Shader};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseButton;
use std::ffi::{CStr, CString};
use vector::TVector3;
use std::time::{Duration, SystemTime};

const SCR_WIDTH: u32 = 1200;
const SCR_HEIGHT: u32 = 900;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("argument not correct");
        return
    }
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
    let mut models: ModelGroup = parse::parse(&args[1]);
    models.texture_on(&shader_program);


    let mut camera = Camera::new(
        TVector3::from([0., 0., 10.]),
        TVector3::from([0., 0., 0.]),
        TVector3::from([0., 1., 0.]),
    );

    let text_id = models.init_textures(&shader_program);
    let mut mouse = Mouse {
        // pressed: false,
        last_x: 0,
        last_y: 0,
    };

    match now.elapsed() {
        Ok(elapsed) => {
            println!("parse time: {}ms", elapsed.as_millis());
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }

    while process_events(&mut event_pump, &mut camera, &mut models, &shader_program, &mut mouse) {
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
        unsafe {
            for (i, tid) in text_id.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);
                gl::BindTexture(gl::TEXTURE_2D, tid.clone());
            }
        }
        models.display(&shader_program);
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

struct Mouse {
    pub last_x: i32,
    pub last_y: i32,
}

fn is_pressed(event_pump: &mut sdl2::EventPump, code: Scancode) -> bool {
    event_pump.keyboard_state().is_scancode_pressed(code)
}

fn is_a_pressed(e: &sdl2::EventPump) -> bool {
    e.mouse_state().left()
}

fn is_left_pressed(e: &sdl2::EventPump) -> bool {
    e.mouse_state().is_mouse_button_pressed(MouseButton::Left)
}

fn get_mouse_pos(e: &sdl2::EventPump) -> (i32, i32) {
    (e.mouse_state().x(), e.mouse_state().y())
}

fn process_events(event_pump: &mut sdl2::EventPump, camera: &mut Camera, models: &mut ModelGroup, shader_program: &Program, mouse: &mut Mouse) -> bool {
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
            Event::MouseButtonDown { timestamp: _, window_id: _, which: _, mouse_btn: _, clicks:_, x, y } => {
                mouse.last_x = x;
                mouse.last_y = y;
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
    if is_pressed(event_pump, Scancode::Equals) {
        let s = models.get_scale();
        models.scale(s[0] * 1.1, s[1] * 1.1,  s[2] * 1.1);

    }
    if is_pressed(event_pump, Scancode::Minus) {
        let s = models.get_scale();
        models.scale(s[0] * 0.9, s[1] * 0.9,  s[2] * 0.9);
    }

    if is_a_pressed(&event_pump) {

    }
    if is_left_pressed(&event_pump) {
        let (x, y) = get_mouse_pos(event_pump);
        let scale = 0.1;
        let diff_x = (x - mouse.last_x) as f32 * scale;
        let diff_y = (y - mouse.last_y) as f32 * scale;
        camera.look_dir(diff_x, diff_y);
        mouse.last_x = x;
        mouse.last_y = y;
    }

    return true;
}

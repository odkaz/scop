extern crate gl;
extern crate image;
extern crate nalgebra_glm as glm;
extern crate sdl2;

pub mod buffer;
mod parse;
mod mvp;
pub mod render_gl;
mod texture;

use buffer::Buffer;
use render_gl::{Shader, Program};
use std::ffi::CString;
use std::time::Duration;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

fn main() {
    let vertices = parse::parse("resources/triangle.obj");
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
    unsafe {
        color_buf.enable();
    }
    let textures: [f32; 6] = [
        1.0, 0.0,
        0.0, 0.0,
        0.0, 1.0
    ];
    let text_buf = Buffer::new();
    text_buf.bind(&Vec::from(textures));
    unsafe {
        text_buf.enable_texture();
    }
    texture::texture();

    unsafe {
        gl::BindVertexArray(0); // Call this when all the bindings are done
    }

    'main: loop {
        //event handler
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        let (w, h) = window.size();
        unsafe {
            gl::Viewport(0, 0, w as i32, h as i32);
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        }

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        shader_program.set_used();

        // pass uniform to shader
        // unsafe {
        //     let c_str = CString::new("mvp").unwrap();
        //     let uniformLoc = gl::GetUniformLocation(shader_program.id(), c_str.as_ptr());
        //     gl::UniformMatrix4fv(uniformLoc, 1, gl::FALSE, glm::value_ptr(&mvp).as_ptr());
        // }

        // render triangles
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                (vertices.len() / 3) as i32,// number of indices to be rendered
            );
        }
        window.gl_swap_window();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

use crate::buffer::Buffer;
use crate::matrix::{Matrix, TMatrix4};
// use crate::parse;
use crate::texture;
// use crate::vector::Vector;
use crate::render_gl::Program;
use gl;
use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct Model {
    vertices: Vec<f32>,
    uvs: Vec<f32>,
    normals: Vec<f32>,
    vao: gl::types::GLuint,
    pos: [f32; 3],
    rot: [f32; 3],
    scl: [f32; 3],
    // cen: [f32; 3],
}

impl Model {
    pub fn init(v: Vec<f32>, uvs: Vec<f32>, norms: Vec<f32>) -> Model {
        // let normals = Self::create_normal(&v);
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            let vertex_buf = Buffer::new(0);
            vertex_buf.bind(&v);
            vertex_buf.enable();
        }
        let colors: [f32; 9] = [
            1., 0.5, 0.0, // left
            0.5, 0.5, 0.0, // right
            0.5, 0.5, 0.0, // top
        ];
        let color_buf = Buffer::new(1);
        color_buf.bind(&Vec::from(colors));
        color_buf.enable();
        let text_buf = Buffer::new(2);
        text_buf.bind(&uvs);
        text_buf.enable_texture();
        texture::texture();
        let norm_buf = Buffer::new(3);
        norm_buf.bind(&norms);
        norm_buf.enable();
        unsafe {
            gl::BindVertexArray(0); // Call this when all the bindings are done
        }
        Model {
            vertices: v,
            uvs,
            normals: norms,
            vao,
            pos: [0.0_f32; 3],
            rot: [0.0_f32; 3],
            scl: [1.0_f32; 3],
        }
    }

    pub fn delete() {
        unsafe {
            gl::BindVertexArray(0); // Call this when all the bindings are done
        }
    }

    fn create_center(v: &Vec<f32>) -> [f32; 3] {
        let mut sum = [0.; 3];
        for (i, v) in v.iter().enumerate() {
            sum[i % 3] = sum[i % 3] + v;
        }
        let x = sum[0] / (v.len() / 3) as f32;
        let y = sum[1] / (v.len() / 3) as f32;
        let z = sum[2] / (v.len() / 3) as f32;
        [x, y, z]
    }

    pub fn display(&self, shader_program: &Program) {
        unsafe {
            shader_program.set_used();
            shader_program.set_mat4(c_str!("model"), &self.get_model());
            gl::BindVertexArray(self.get_vao());
            gl::DrawArrays(gl::TRIANGLES, 0, (self.get_vertices().len() / 3) as i32);
        }
    }

    pub fn move_x(&mut self, scale: f32) {
        let mut buf = self.pos.clone();
        buf[0] = buf[0] + scale;
        self.pos = buf;
    }

    pub fn move_y(&mut self, scale: f32) {
        let mut buf = self.pos.clone();
        buf[1] = buf[1] + scale;
        self.pos = buf;
    }

    pub fn move_z(&mut self, scale: f32) {
        let mut buf = self.pos.clone();
        buf[2] = buf[2] + scale;
        self.pos = buf;
    }



    pub fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    pub fn get_normals(&self) -> Vec<f32> {
        self.normals.clone()
    }

    pub fn get_vao(&self) -> gl::types::GLuint {
        self.vao
    }

    pub fn get_trans(&self) -> [f32; 3] {
        self.pos
    }

    pub fn set_trans(&mut self, x: f32, y: f32, z: f32) {
        self.pos = [x, y, z];
    }

    pub fn set_rot(&mut self, x: f32, y: f32, z: f32) {
        self.rot = [x, y, z];
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scl = [x, y, z];
    }

    fn translation(&self) -> TMatrix4<f32> {
        Matrix::translation(self.pos[0], self.pos[1], self.pos[2])
    }

    fn rotation(&self) -> TMatrix4<f32> {
        Matrix::rotation(self.rot[0], self.rot[1], self.rot[2])
    }

    fn scale(&self) -> TMatrix4<f32> {
        Matrix::scale(self.scl[0], self.scl[1], self.scl[2])
    }

    pub fn get_model(&self) -> TMatrix4<f32> {
        // let mat_center = Matrix::translation(-self.c[0], -self.c[1], -self.c[2]);
        // self.translation() * self.rotation() * self.scale() * mat_center
        self.translation() * self.rotation() * self.scale()
    }
}

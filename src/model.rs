use gl;
use crate::parse;
use crate::buffer::{Buffer};
use crate::matrix::{Matrix, TMatrix4};

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<f32>,
    pub vao: gl::types::GLuint,
    t: [f32; 3],
    r: [f32; 3],
    s: [f32; 3],
}

impl Model {
    pub fn new(path: &str) -> Model {
        let (v, vao) = Self::load_vertex(path);
        Model {
            vertices: v,
            vao,
            t: [0.0_f32; 3],
            r: [0.0_f32; 3],
            s: [1.0_f32; 3],
        }
    }

    pub fn delete() {
        unsafe {
            gl::BindVertexArray(0); // Call this when all the bindings are done
        }
    }

    fn load_vertex(path: &str) -> (Vec<f32>, gl::types::GLuint) {
        let vertices = parse::parse(path);
        let mut vao: gl::types::GLuint = 0;
    
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            let vertex_buf = Buffer::new(0);
            vertex_buf.bind(&vertices);
            vertex_buf.enable();
        }
        let colors: [f32; 9] = [
            1., 0.5, 0.0, // left
            0.5, 0.5, 0.0, // right
            0.5,  0.5, 0.0  // top
        ];
        let color_buf = Buffer::new(1);
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
    
        // unsafe {
        //     gl::BindVertexArray(0); // Call this when all the bindings are done
        // }
        (vertices, vao)
    }

    pub fn get_vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    pub fn get_vao(&self) -> gl::types::GLuint {
        self.vao
    }

    pub fn set_trans(&mut self, x: f32, y: f32, z: f32) {
        self.t = [x, y, z];
    }

    pub fn set_rot(&mut self, x: f32, y: f32, z: f32) {
        self.r = [x, y, z];
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.s = [x, y, z];
    }

    fn translation(&self) -> TMatrix4<f32> {
        Matrix::translation(self.t[0], self.t[1], self.t[2])
    }

    fn rotation(&self) -> TMatrix4<f32> {
        Matrix::rotation(self.r[0], self.r[1], self.r[2])
    }

    fn scale(&self) -> TMatrix4<f32> {
        Matrix::scale(self.s[0], self.s[1], self.s[2])
    }

    pub fn get_model(&self) -> TMatrix4<f32> {
        self.translation() * self.rotation() * self.scale()
    }
}
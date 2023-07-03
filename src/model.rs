use crate::buffer::Buffer;
use crate::matrix::{Matrix, TMatrix4};
use crate::parse;
use crate::texture;
use crate::vector::Vector;
use gl;

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<f32>,
    pub uvs: Vec<f32>,
    pub normals: Vec<f32>,
    pub vao: gl::types::GLuint,
    c: [f32; 3],
    t: [f32; 3],
    r: [f32; 3],
    s: [f32; 3],
}

impl Model {
    pub fn new(path: &str) -> Model {
        let (v, uvs, n, vao) = Self::load_vertex(path);
        Model {
            vertices: v.clone(),
            uvs,
            normals: n,
            vao,
            c: Self::create_center(&v),
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

    fn create_normal(v: &Vec<f32>) -> Vec<f32> {
        let mut res = Vec::new();
        for row in 0..v.len() / 9 {
            let i = row * 9;
            let p0 = [v[i], v[i + 1], v[i + 2]];
            let p1 = [v[i + 3], v[i + 4], v[i + 5]];
            let p2 = [v[i + 6], v[i + 7], v[i + 8]];
            let v0 = Vector::from(p0) - Vector::from(p1);
            let v1 = Vector::from(p0) - Vector::from(p2);
            let tmp = Vector::cross_product(&v0, &v1).as_vec();
            for _ in 0..3 {
                res.push(tmp[0]);
                res.push(tmp[1]);
                res.push(tmp[2]);
            }
        }
        res
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

    fn load_vertex(path: &str) -> (Vec<f32>, Vec<f32>, Vec<f32>, gl::types::GLuint) {
        let (vertices, uvs) = parse::parse(path);
        let normals = Self::create_normal(&vertices);
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
            0.5, 0.5, 0.0, // top
        ];
        let color_buf = Buffer::new(1);
        color_buf.bind(&Vec::from(colors));
        color_buf.enable();
        // let textures: [f32; 12] = [
        //     1.0, 1.0,
        //     1.0, 0.0,
        //     0.0, 1.0,
        //     1.0, 0.0,
        //     0.0, 0.0,
        //     0.0, 1.0
        // ];
        let text_buf = Buffer::new(2);
        // text_buf.bind(&Vec::from(textures));
        text_buf.bind(&uvs);
        text_buf.enable_texture();
        texture::texture();

        let norm_buf = Buffer::new(3);
        norm_buf.bind(&normals);
        norm_buf.enable();

        unsafe {
            gl::BindVertexArray(0); // Call this when all the bindings are done
        }
        (vertices, uvs, normals, vao)
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
        let mat_center = Matrix::translation(-self.c[0], -self.c[1], -self.c[2]);
        self.translation() * self.rotation() * self.scale() * mat_center
    }
}

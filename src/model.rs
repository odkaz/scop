use crate::buffer::Buffer;
use crate::matrix::{Matrix, TMatrix4};
// use crate::parse;
use crate::texture;
// use crate::vector::Vector;
use crate::render_gl::Program;
use gl;
use std::ffi::{CStr, CString};

#[derive(Debug, Clone)]
pub struct ModelGroup {
    models: Vec<Model>,
    pos: [f32; 3],
    rot: [f32; 3],
    scl: [f32; 3],
    center: [f32; 3],
    text_bool: bool,
    textures: Vec<String>,
}

impl ModelGroup {
    pub fn new(models: Vec<Model>) -> Self {
        let mut verticies = Vec::new();
        let mut textures = Vec::new();
        for m in &models {
            verticies.append(&mut m.get_vertices());
            if !(textures.iter().any(|c| c == &m.get_texture())) {
                textures.push(m.get_texture());
            }
        }
        // init_textures()
        ModelGroup {
            models: models,
            pos: [0.0_f32; 3],
            rot: [0.0_f32; 3],
            scl: [1.0_f32; 3],
            center: Self::create_center(&verticies),
            text_bool: true,
            textures,
        }
    }

    pub fn display(&self, shader_program: &Program) {
        let name_list = [
            "tex1Intensity",
            "tex2Intensity",
            "tex3Intensity",
            "tex4Intensity",
            "tex5Intensity",
        ];
        //activate all textures
        unsafe {
            for m in &self.models {
                for (i, item) in self.textures.iter().enumerate() {
                    let cname = CString::new(name_list[i]).expect("CString::new failed");

                    if item == &m.get_texture() {
                        shader_program.set_float(&cname, 1.0);
                    } else {
                        shader_program.set_float(&cname, 0.0);
                    }
                }
                m.display(shader_program, self.center);
            }
        }

    }

    pub fn init_textures(&self, shader_program: &Program) -> Vec<u32> {
        let mut id = Vec::new();
        let mut name_list = [
            "texture1",
            "texture2",
            "texture3",
            "texture4",
            "texture5",
        ];

        for (i, t) in self.textures.clone().iter().enumerate() {
            let tex = texture::texture(&t);
            unsafe {
                let cname = CString::new(name_list[i]).expect("CString::new failed");
                let tex_loc  = gl::GetUniformLocation( shader_program.id(), cname.as_ptr());
                gl::Uniform1i(tex_loc, i.try_into().unwrap());
            }
            id.push(tex);
        }
        id
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

    pub fn move_x(&mut self, scale: f32) {
        for m in &mut self.models {
            m.move_x(scale);
        }
    }

    pub fn move_y(&mut self, scale: f32) {
        for mut m in &mut self.models {
            m.move_y(scale);
        }
    }

    pub fn move_z(&mut self, scale: f32) {
        for mut m in &mut self.models {
            m.move_z(scale);
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.pos = [x, y, z];
        for m in &mut self.models {
            m.set_trans(x, y, z);
        }
    }

    pub fn rotate(&mut self, x: f32, y: f32, z: f32) {
        self.rot = [x, y, z];
        for m in &mut self.models {
            m.set_rot(x, y, z);
        }
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.scl = [x, y, z];
        for m in &mut self.models {
            m.set_scale(x, y, z);
        }
    }

    pub fn get_scale(&self) -> [f32; 3] {
        self.scl
    }

    pub fn texture_on(&mut self, shader_program: &Program) {
        for m in &mut self.models {
            m.set_texture_intensity(shader_program, 1.0);
        }
        self.text_bool = true;
    }

    pub fn invert_texture(&mut self, shader_program: &Program) {
        if self.text_bool {
            for m in &mut self.models {
                m.set_texture_intensity(shader_program, 0.0);
            }
            self.text_bool = false;
        } else {
            for m in &mut self.models {
                m.set_texture_intensity(shader_program, 1.0);
            }
            self.text_bool = true;
        }

    }
}


#[derive(Debug, Clone)]
pub struct Model {
    vertices: Vec<f32>,
    uvs: Vec<f32>,
    normals: Vec<f32>,
    vao: gl::types::GLuint,
    pos: [f32; 3],
    rot: [f32; 3],
    scl: [f32; 3],
    texture: String,
    // cen: [f32; 3],
}

impl Model {
    pub fn init(v: Vec<f32>, uvs: Vec<f32>, norms: Vec<f32>, tex: String) -> Model {
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
        //texture::texture(&tex);
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
            texture: tex,
        }
    }

    pub fn delete() {
        unsafe {
            gl::BindVertexArray(0); // Call this when all the bindings are done
        }
    }

    pub fn display(&self, shader_program: &Program, center: [f32; 3]) {
        unsafe {
            shader_program.set_used();
            shader_program.set_mat4(c_str!("model"), &self.get_model(center));
            gl::BindVertexArray(self.get_vao());
            gl::DrawArrays(gl::TRIANGLES, 0, (self.get_vertices().len() / 3) as i32);
        }
    }

    pub fn set_texture_intensity(&self, shader_program: &Program, scale: f32) {
        unsafe {
            shader_program.set_used();
            shader_program.set_float(c_str!("textIntensity"), scale);
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

    pub fn get_texture(&self) -> String {
        self.texture.clone()
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

    pub fn get_model(&self, center: [f32; 3]) -> TMatrix4<f32> {
        let mat_center = Matrix::translation(-center[0], -center[1], -center[2]);
        self.translation() * self.rotation() * self.scale() * mat_center
        // self.translation() * self.rotation() * self.scale()
    }
}

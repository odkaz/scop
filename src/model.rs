use gl;

pub struct Model {
    pub vertices: Vec<f32>,
    pub vao: gl::types::GLuint,
    t: [f32; 3],
    r: [f32; 3],
    s: [f32; 3],
}

impl Model {
    pub fn new(path: &str) -> Model {
        let (v, vao) = load_vertex(path);
        Model {
            vertices: v,
            vao,
            t: [f32::zero(); 3],
            r: [f32::zero(); 3],
            s: [f32::zero(); 3],
        }
    }

    pub fn delete() {
        unsafe {
            gl::BindVertexArray(0); // Call this when all the bindings are done
        }
    }

    fn load_vertex(path: &str) -> (Vec<f32>, gl::types::GLuint) {
        let vertices = parse::parse(path);
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
        Matrix::translation(t[0], t[1], t[2])
    }

    fn rotation(&self) -> TMatrix4<f32> {
        Matrix::rotation(t[0], t[1], t[2])
    }

    fn scale(&self) -> TMatrix4<f32> {
        Matrix::scale(t[0], t[1], t[2])
    }

    pub fn get_model(&self) -> TMatrix4<f32> {
        self.translation() * self.rotation() * self.scale()
    }
}
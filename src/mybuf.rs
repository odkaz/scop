use gl;
use self::gl::types::*;
static mut LOC: u32 = 0;

pub struct Buffer {
    _vbo: GLuint,
    _loc: u32,
}

impl Buffer {
    pub fn new() -> Buffer {
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
            let buf = Buffer {
                _vbo: vbo,
                _loc: LOC,
            };
            println!("created new buf to {}", LOC);
            LOC = LOC + 1;
            return buf
        }
    }

    pub fn bind(&self, data: &Vec<f32>) {
        println!("bind{:?}", data);
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self._vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (data.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
                data.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW,                                    // usage
            );
            // gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }
    }

    pub fn enable(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self._vbo);
            gl::EnableVertexAttribArray(self._loc); // this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                self._loc,         // index of the generic vertex attribute ("layout (location = 0)")
                3,         // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
                std::ptr::null(),                                     // offset of the first component
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
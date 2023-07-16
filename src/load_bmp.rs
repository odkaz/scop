use std::fs::File;
use std::io::prelude::*;
use std::io::{self};

pub struct Bitmap {
    path: String,
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Bitmap {
    // pub fn load() -> bmp::Image {
    //     let img: bmp::Image = bmp::open("resources/Textures-16.bmp").unwrap_or_else(|e| {
    //         panic!("Failed to open: {}", e);
    //     });
    //     return img
    // }
    pub fn rotate_180(&mut self) {
        println!("width{} height{}", self.width, self.height);
        let mut rgb = Vec::new();
        for h in 0..self.height as usize {
            for w in 0..self.width as usize {

                let pad = ((self.height - 1) as usize - h) * self.width as usize + w;
                let inv = (self.height * self.width - 1) as usize - pad;
                // println!("w {} h{}", w, h);
                // println!("pad{} inv{}", pad, inv);
                let r = self.data[3 * inv];
                let g = self.data[3 * inv + 1];
                let b = self.data[3 * inv + 2];
                // println!("R:{} G:{} B:{}", r, g, b);
                rgb.push(r);
                rgb.push(g);
                rgb.push(b);
            }
        }
        self.data = rgb;
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn get_data(&self) -> &Vec<u8> {
        return &self.data;
    }
    pub fn read_file() {}

    pub fn open(path: &String) -> Result<Bitmap, io::Error> {
        let mut file = File::open(path)?;
        let mut header_buf = [0; 54];
        file.read_exact(&mut header_buf)
            .expect("failed to read file");
        if header_buf[0] != 'B' as u8 || header_buf[1] != 'M' as u8 {
            panic!("The entered file does not have appropriate BMP header");
        }
        let mut data_pos: u32 = header_buf[10] as u32;
        // let mut image_size: u32 = 0;
        let width: u32 = u32::from_ne_bytes(header_buf[18..22].try_into().unwrap());
        let height: u32 = u32::from_ne_bytes(header_buf[22..26].try_into().unwrap());

        // let per_pixel = u16::from_ne_bytes(header_buf[28..30].try_into().unwrap());
        // println!("per:{}", per_pixel);
        if data_pos == 0 {
            data_pos = 54;
        }
        // if image_size == 0 {
        //     image_size = width * height * 3;
        // }
        // for (i, b) in header_buf.iter().enumerate() {
        //     println!("i: {}, b: {}", i, b);
        // }
        let mut content = Vec::new();
        file.read_to_end(&mut content).expect("failed to read");
        let s = &content.as_slice()[data_pos as usize - 54..];
        let mut rgb = Vec::new();

        for h in 0..height {
            for w in 0..width {
                let pad = (h * width + w) as usize;
                // let pad = ((height - 1 - h) * width + (width - 1 - w)) as usize;
                let r = s[3 * pad + 2];
                let g = s[3 * pad + 1];
                let b = s[3 * pad];
                rgb.push(r);
                rgb.push(g);
                rgb.push(b);
            }
        }

        // for h in 0..height as usize {
        //     for w in 0..width as usize {
        //         let pad = ((height - 1) as usize - h) * width as usize + w;
        //         let r = s[3 * pad + 2];
        //         let g = s[3 * pad + 1];
        //         let b = s[3 * pad];
        //         // println!("R:{} G:{} B:{}", r, g, b);
        //         rgb.push(r);
        //         rgb.push(g);
        //         rgb.push(b);
        //     }
        // }
        Ok(Bitmap {
            path: path.to_string(),
            data: rgb,
            // data: Vec::from(s),
            width: width,
            height: height,
        })
    }

    pub fn print(&self) {
        println!("path {}", self.path);
    }
}

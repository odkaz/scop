use std::fs::{File, metadata};
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
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn get_data(&self) -> &Vec<u8> {
        return &self.data
    }
    pub fn read_file() {}

    pub fn open(path: &String) -> Result<Bitmap, io::Error> {
        let x = metadata(path)?.len();

        let mut file = File::open(path)?;
        let mut header_buf = [0; 54];
        file.read_exact(&mut header_buf);
        if header_buf[0] != 'B' as u8 || header_buf[1] != 'M' as u8 {
            panic!("The entered file does not have appropriate BMP header");
        }
        let mut dataPos: u32 = header_buf[10] as u32;
        let mut imageSize: u32 = 0;
        let width: u32 = u32::from_ne_bytes(header_buf[18..22].try_into().unwrap());
        let height: u32 = u32::from_ne_bytes(header_buf[22..26].try_into().unwrap());
        if dataPos == 0 {
            dataPos = 54;
        }
        if imageSize == 0 {
            imageSize = width * height * 3;
        }
        let mut content = Vec::new();
        file.read_to_end(&mut content).expect("failed to read");
        let s = &content.as_slice()[dataPos as usize - 54..];
        let mut rgb = Vec::new();

        for h in 0..height as usize {
            for w in 0..width as usize {
                let pad = ((height - 1) as usize - h) * width as usize + w;
                let r = s[3 * pad + 2];
                let g = s[3 * pad + 1];
                let b = s[3 * pad];
                rgb.push(r);
                rgb.push(g);
                rgb.push(b);
            }
        }
        Ok(Bitmap {
            path: path.to_string(),
            data: rgb,
            width: width,
            height: height,
        })
    }

    pub fn print(&self) {
        println!("path {}", self.path);
    }
}

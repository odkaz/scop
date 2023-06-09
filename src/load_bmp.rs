use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};

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

    pub fn get_data(&self) -> &Vec<u8> {
        return &self.data
    }
    pub fn read_file() {}

    pub fn open(path: &String) -> Result<Bitmap, io::Error> {
        let mut file = File::open(path)?;
        let mut header_buf = [0; 54];
        file.read_exact(&mut header_buf);
        println!("{:?}", header_buf);
        println!("0{}, 1{}", header_buf[0] as char, header_buf[1] as char);
        if header_buf[0] != 'B' as u8 || header_buf[1] != 'M' as u8 {
            panic!("The entered file does not have appropriate BMP header");
        }
        let mut dataPos: u32 = 0;
        let mut imageSize: u32 = 0;
        let width: u32 = u32::from_ne_bytes(header_buf[18..22].try_into().unwrap());
        let height: u32 = u32::from_ne_bytes(header_buf[22..26].try_into().unwrap());
        println!("pos{}, size{}", dataPos, imageSize);
        println!("0{}, 1{}", header_buf[18] as u32, header_buf[22] as u32);

        println!("ans{}", width * height * 3);
        if dataPos == 0 {
            dataPos = 54;
        }
        if imageSize == 0 {
            imageSize = width * height * 3;
        }
        println!("pos{}, size{}", dataPos, imageSize);

        println!("w{}, h{}", width, height);

        let mut content = Vec::new();
        file.read_to_end(&mut content);

        Ok(Bitmap {
            path: path.to_string(),
            data: content,
            width: width,
            height: height,
        })
    }

    pub fn print(&self) {
        println!("path {}", self.path);
    }
}

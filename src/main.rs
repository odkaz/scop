use std::fs::File;
use std::io::{ self, BufRead, BufReader};
#[derive(Debug)]
struct Point(f32, f32, f32);
#[derive(Debug)]
struct Face(Vec<i32>);

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn get_point(line: String) -> Point {
    let mut tmp = Vec::new();
    for byte in line.split_whitespace() {
        let t: f32 = match byte.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        tmp.push(t);
    }
    Point(tmp[0], tmp[1], tmp[2])
}

fn get_face(line: String) -> Face {
    let mut tmp = Vec::new();
    for byte in line.split_whitespace() {
        let t: i32 = match byte.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        tmp.push(t);
    }
    Face(tmp)
}

fn main() {
    let file_path = "./resources/42.obj";
    let mut points = Vec::new();
    let mut faces = Vec::new();

    let lines = read_lines(file_path.to_string());
    for line in lines {
        let str1 = line.unwrap();
        let id = str1.chars().nth(0).unwrap();

        match id {
            'v' => {
                let p = get_point(str1);
                points.push(p);
            },
            'f' => {
                let f = get_face(str1);
                faces.push(f);
            },
            _ => (),
        }
    }
    println!("{:#?}", points);
    println!("{:#?}", faces);
}

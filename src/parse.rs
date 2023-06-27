use std::fs::File;
use std::io::{ self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn get_point(line: String) -> Vec<f32> {
    let mut point = Vec::new();
    for byte in line.split_whitespace() {
        let t: f32 = match byte.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        point.push(t);
    }
    point
}

fn get_face(line: String) -> Vec<usize> {
    let mut face = Vec::new();
    for byte in line.split_whitespace() {
        let parts: Vec<&str> = byte.split("/").collect();
        let t: usize = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        face.push(t);
    }
    face
}

pub fn parse(file_path: &str) -> Vec<f32> {
    let mut points = Vec::new();
    let mut faces = Vec::new();

    let lines = read_lines(file_path.to_string());
    for line in lines {
        let str1 = line.unwrap();
        let id = match str1.chars().nth(0) {
            None => continue,
            Some(c) => c,
        };

        match id {
            'v' => {
                let p = get_point(str1);
                points.push(p);
            },
            'f' => {
                let mut f = get_face(str1);
                if f.len() == 3 {
                    faces.push(f);
                } else if f.len() == 4 {
                    faces.push(Vec::from([f[0], f[1], f[2]]));
                    faces.push(Vec::from([f[0], f[2], f[3]]));
                }
            },
            _ => (),
        }
    }

    let mut vertices = Vec::new();
    for face in faces {
        for f in face {
            for point in &points[f - 1] {
                vertices.push(*point);
            }
        }
    }
    return vertices
}

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn get_point(line: Vec<&str>) -> Vec<f32> {
    let mut point = Vec::new();
    for byte in line {
        let t: f32 = match byte.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        point.push(t);
    }
    point
}

fn get_face(line: Vec<&str>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut face: Vec<usize> = Vec::new();
    let mut uv: Vec<usize> = Vec::new();
    let mut vn: Vec<usize> = Vec::new();

    for byte in line {
        let parts: Vec<&str> = byte.split("/").collect();
        for (i, p) in parts.iter().enumerate() {
            match p.parse() {
                Ok(num) => {
                    if i == 0 {
                        face.push(num);
                    }
                    if i == 1 {
                        uv.push(num);
                    }
                    if i == 2 {
                        vn.push(num);
                    }
                },
                Err(_) => (),
            }
        }
    }
    (face, uv, vn)
}

pub fn parse(file_path: &str) -> (Vec<f32>, Vec<f32>) {
    let mut points = Vec::new();
    let mut faces = Vec::new();
    let mut uvs = Vec::new();
    let mut uv_points = Vec::new();
    let mut vns = Vec::new();
    let mut vts = Vec::new();

    let lines = read_lines(file_path.to_string());
    for line in lines {
        let str1 = line.unwrap();
        let s: Vec<&str> = str1.split_whitespace().collect();
        if s.len() == 0 {
            continue;
        }
        match s[0] {
            "v" => {
                let p = get_point(s);
                points.push(p);
            }
            "f" => {
                let (f, uv, vn) = get_face(s);
                if f.len() == 3 {
                    faces.push(f);
                    if uv.len() != 0 {
                        uv_points.append(&mut uv.clone());
                    } else {
                        uvs.append(&mut Vec::from([0.0, 0.0, 0.5, 1.0, 1.0, 0.0]));
                    }
                } else if f.len() == 4 {
                    faces.push(Vec::from([f[0], f[1], f[2]]));
                    faces.push(Vec::from([f[0], f[2], f[3]]));
                    if uv.len() != 0 {
                        uv_points.append(&mut Vec::from([uv[0], uv[1], uv[2]]));
                        uv_points.append(&mut Vec::from([uv[0], uv[2], uv[3]]));
                    } else {
                        uvs.append(&mut Vec::from([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]));
                        uvs.append(&mut Vec::from([0.0, 0.0, 1.0, 1.0, 1.0, 0.0]));
                    }
                } else if f.len() > 4 {
                    println!("f len 4+");
                }
            }
            "vn" => {
                let vn = get_point(s);
                vns.push(vn);
            }
            "vt" => {
                let vt = get_point(s);
                vts.push(vt);
            }
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

    for u in uv_points {
        for p in &vts[u - 1] {
            // println!("u{}",u);
            uvs.push(p.clone());
        }
    }
    (vertices, uvs)
}

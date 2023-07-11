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

fn parse_f(line: Vec<&str>) -> (Vec<usize>, Vec<usize>, Vec<f32>) {
    let (f, uv, n) = get_face(line);
    let mut faces = Vec::new();
    let mut uv_points = Vec::new();
    let mut uvs = Vec::new();
    for i in 0..f.len() - 2 {
        faces.push(f[0] - 1);
        faces.push(f[i + 1] - 1);
        faces.push(f[i + 2] - 1);
        if uv.len() != 0 {
            uv_points.push(uv[0] - 1);
            uv_points.push(uv[i + 1] - 1);
            uv_points.push(uv[i + 2] - 1);
        } else {
            uvs.append(&mut Vec::from([0.0, 0.0, 0.5, 1.0, 1.0, 0.0]));
        }
    }
    (faces, uv_points, uvs)
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
                let (f, u, mut uv) = parse_f(s);
                faces.push(f);
                uv_points.push(u);
                uvs.append(&mut uv);
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
            for point in &points[f] {
                vertices.push(*point);
            }
        }
    }

    for uvp in uv_points {
        for u in uvp {
            for p in &vts[u] {
                uvs.push(p.clone());
            }
        }
    }

    (vertices, uvs)
}

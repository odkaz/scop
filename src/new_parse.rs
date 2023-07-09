use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::new_model::Model;
use crate::vector::{Vector, TVector3};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

fn parse_v(line: Vec<&str>) -> TVector3<f32> {
    let mut p = Vec::new();
    for byte in line {
        let t: f32 = match byte.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        p.push(t);
    }
    Vector::from([p[0], p[1], p[2]])
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


fn create_normal(p0: TVector3<f32>, p1: TVector3<f32>, p2: TVector3<f32>) -> Vec<f32> {
    let mut res = Vec::new();
    let v0 = Vector::from(p0.clone()) - Vector::from(p1);
    let v1 = Vector::from(p0) - Vector::from(p2);
    let mut n = Vector::cross_product(&v0, &v1).as_vec();
    for _ in 0..3 {
        res.append(&mut n);
    }
    res
}

//return vertice, uv and normal
fn parse_f(line: Vec<&str>, v: &mut Vec<TVector3<f32>>, vt: &mut Vec<Vec<f32>>, vn: &mut Vec<Vec<f32>>) -> (Vec<f32>, Vec<f32>, Vec<f32>){
    let mut vertices: Vec<f32> = Vec::new();
    let mut uvs: Vec<f32> = Vec::new();
    let mut vns: Vec<f32> = Vec::new();
    let (f, uv, n) = get_face(line);
    for i in 0..f.len() - 2 {
        vertices.append(&mut v[f[0] - 1].as_vec());
        vertices.append(&mut v[f[i + 1] - 1].as_vec());
        vertices.append(&mut v[f[i + 2] - 1].as_vec());
        if uv.len() != 0 {
            uvs.append(&mut vt[uv[0] - 1].clone());
            uvs.append(&mut vt[uv[i + 1] - 1].clone());
            uvs.append(&mut vt[uv[i + 2] - 1].clone());
        } else {
            uvs.append(&mut Vec::from([0.0, 0.0, 0.5, 1.0, 1.0, 0.0]));
        }
        if n.len() != 0 {
            vns.append(&mut vn[n[0] - 1].clone());
            vns.append(&mut vn[n[i + 1] - 1].clone());
            vns.append(&mut vn[n[i + 2] - 1].clone());
        } else {
            vns.append(&mut create_normal(v[f[0] - 1].clone(), v[f[i + 1] - 1].clone(), v[f[i + 2] - 1].clone()));
        }
    }
    (vertices, uvs, vns)
}

fn get_mtl_path(line: Vec<&str>) -> Vec<&str> {
    line[1..].to_vec()
}

fn get_g_name(s: Vec<&str>) -> Result<&str, &str> {
    if s.len() != 2 {
        return Err("no g name found");
    }
    Ok(s[1])
}

pub fn parse(file_path: &str) -> Vec<Model> {
    let mut v = Vec::new();
    let mut vns = Vec::new();
    let mut vts = Vec::new();
    let mut vertices = Vec::new();
    let mut uvs = Vec::new();
    let mut norms = Vec::new();
    // let mut mtl_paths = Vec::new();
    let mut models = Vec::new();
    let mut g_name: &str;

    let lines = read_lines(file_path.to_string());
    for line in lines {
        let str1 = line.unwrap();
        let s: Vec<&str> = str1.split_whitespace().collect();
        if s.len() == 0 {
            continue;
        }
        match s[0] {
            "v" => {
                let p = parse_v(s);
                v.push(p);
            }
            "vn" => {
                let vn = get_point(s);
                vns.push(vn);
            }
            "vt" => {
                let vt = get_point(s);
                vts.push(vt);
            }
            "f" => {
                let (mut verts, mut uv, mut n) = parse_f(s, &mut v.clone(), &mut vts.clone(), &mut vns.clone());
                vertices.append(&mut verts);
                uvs.append(&mut uv);
                norms.append(&mut n);
            }
            "g" => {
                if vertices.len() != 0 {
                    let model = Model::init(vertices, uvs, norms);
                    models.push(model);
                    vertices = Vec::new();
                    uvs = Vec::new();
                    norms = Vec::new();
                }
                g_name = get_g_name(s).unwrap();
            }
            "mtllib" => {
                // mtl_paths.append(&mut get_mtl_path(s));
            }
            "usemtl" => {

            }
            _ => (),
        }
    }
    if vertices.len() != 0 {
        // println!("v{:?}, {}", vertices, vertices.len());
        // println!();
        // println!("u{:?}, {}", uvs, uvs.len());
        // println!();
        // println!("n{:?}, {}", norms, norms.len());
        // println!();
        let model = Model::init(vertices, uvs, norms);
        models.push(model);
    }
    models
}

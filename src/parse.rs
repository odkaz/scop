use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::model::{Model, ModelGroup};
use crate::vector::{Vector};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    return io::BufReader::new(file).lines();
}

// fn get_point(line: Vec<&str>) -> Vec<f32> {
//     let mut point = Vec::new();
//     for byte in line {
//         let t: f32 = match byte.parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };
//         point.push(t);
//     }
//     point
// }


fn get_point(line: Vec<&str>, num: usize) -> Vec<f32> {
    let mut point = Vec::new();
    for byte in line {
        let t: f32 = match byte.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if point.len() < num {
            point.push(t);
        }
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

fn parse_f(line: Vec<&str>) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let (f, uv, n) = get_face(line);
    let mut faces = Vec::new();
    let mut uv_points = Vec::new();
    let mut n_points = Vec::new();
    for i in 0..f.len() - 2 {
        faces.push(f[0] - 1);
        faces.push(f[i + 1] - 1);
        faces.push(f[i + 2] - 1);
        if uv.len() != 0 {
            uv_points.push(uv[0] - 1);
            uv_points.push(uv[i + 1] - 1);
            uv_points.push(uv[i + 2] - 1);
        }
        if n.len() != 0 {
            n_points.push(n[0] - 1);
            n_points.push(n[i + 1] - 1);
            n_points.push(n[i + 2] - 1);
        }
    }
    (faces, uv_points, n_points)
}


fn create_normal(v: &Vec<f32>) -> Vec<f32> {
    let mut res = Vec::new();
    for i in 0..v.len() / 9 {
        let p0 = [v[i * 9], v[i * 9 + 1], v[i * 9 + 2]];
        let p1 = [v[i * 9 + 3], v[i * 9 + 4], v[i * 9 + 5]];
        let p2 = [v[i * 9 + 6], v[i * 9 + 7], v[i * 9 + 8]];
        let v0 = Vector::from(p0.clone()) - Vector::from(p1);
        let v1 = Vector::from(p0) - Vector::from(p2);
        let n = Vector::cross_product(&v0, &v1).as_vec();
        for _ in 0..3 {
            res.push(n[0]);
            res.push(n[1]);
            res.push(n[2]);
        }
    }
    res
}

fn find_texture(paths: Vec<String>, mtl_name: String) -> String {
    let mut res = String::from("");
    let mut flag = false;
    for path in paths {
        let mut mtl = String::from("resources/mtl/");
        mtl.push_str(&path);
        let lines = read_lines(mtl);
        for line in lines {
            let str1 = line.unwrap();
            let s: Vec<&str> = str1.split_whitespace().collect();
            if s.len() == 0 {
                continue;
            }
            match s[0] {
                "newmtl" => {
                    if s[1] == mtl_name {
                        flag = true;
                    } else {
                        flag = false;
                    }
                }
                "map_Kd" => {
                    if flag {
                        // return s[1].to_string()
                        let mut text = String::from("resources/textures/");
                        text.push_str(s[1]);
                        return text
                    }
                }
                _ => (),
            }
        }
    }
    res
}

struct Group {
    name: String,
    faces: Vec<Vec<usize>>,
    uv_points: Vec<Vec<usize>>,
    n_points: Vec<Vec<usize>>,
    texture: String,
}

pub fn parse(file_path: &str) -> ModelGroup {
    let mut points = Vec::new();
    let mut faces = Vec::new();
    let mut uv_points = Vec::new();
    let mut vns = Vec::new();
    let mut vts = Vec::new();
    let mut g_name:String = String::from("");
    let mut groups: Vec<Group> = Vec::new();
    let mut n_points = Vec::new();
    let mut mtl_paths = Vec::new();
    let mut texture = String::from("");

    let lines = read_lines(file_path.to_string());
    for line in lines {
        let str1 = line.unwrap();
        let s: Vec<&str> = str1.split_whitespace().collect();
        if s.len() == 0 {
            continue;
        }
        match s[0] {
            "v" => {
                let p = get_point(s, 3);
                points.push(p);
            }
            "f" => {
                let (f, u, n) = parse_f(s);
                faces.push(f);
                if u.len() != 0 {
                    uv_points.push(u);
                }
                if n.len() != 0 {
                    n_points.push(n);
                }
            }
            "vn" => {
                let vn = get_point(s, 3);
                vns.push(vn);
            }
            "vt" => {
                let vt = get_point(s, 2);
                vts.push(vt);
            }
            "g" | "o" | "s" => {
                if faces.len() != 0 {
                    if texture == "" {
                        texture = String::from("resources/textures/metal.bmp");
                    }
                    println!("text:{}", texture);
                    let g = Group {
                        faces,
                        name: g_name,
                        uv_points,
                        n_points,
                        texture: texture.clone(),
                    };
                    groups.push(g);
                    faces = Vec::new();
                    uv_points = Vec::new();
                    n_points = Vec::new();
                }
                g_name = s[1].to_string();
            }
            // "s" => {
            //     //same as g
            //     if faces.len() != 0 {
            //         if texture == "" {
            //             texture = String::from("resources/textures/metal.bmp");
            //         }
            //         println!("text:{}", texture);
            //         let g = Group {
            //             faces,
            //             name: g_name,
            //             uv_points,
            //             n_points,
            //             texture: texture.clone(),
            //         };
            //         groups.push(g);
            //         faces = Vec::new();
            //         uv_points = Vec::new();
            //         n_points = Vec::new();
            //     }
            //     g_name = s[1].to_string();
            // }
            "mtllib" => {
                for i in s[1..].iter() {
                    mtl_paths.push(i.to_string());
                }
            }
            "usemtl" => {
                texture = find_texture(mtl_paths.clone(), s[1].to_string());
            }
            _ => (),
        }
    }
    if texture == "" {
        texture = String::from("resources/textures/barbara/skin.bmp");
    }

    let g = Group {
        faces,
        name: g_name,
        uv_points,
        n_points,
        texture,
    };
    groups.push(g);


    let mut models = Vec::new();
    for g in groups {
        let mut vertices = Vec::new();
        for face in g.faces.clone() {
            for f in face {
                for point in &points[f] {
                    vertices.push(*point);
                }
            }
        }

        let mut uvs = Vec::new();
        if g.uv_points.len() != 0 {
            for uvp in g.uv_points.clone() {
                for u in uvp {
                    for p in &vts[u] {
                        uvs.push(p.clone());
                    }
                }
            }
        } else {
            for _ in 0..g.faces.len() {
                uvs.append(&mut Vec::from([0.0, 0.0, 0.5, 1.0, 1.0, 0.0]));
            }
        }

        let mut norms = Vec::new();
        if g.n_points.len() != 0 {
            for np in g.n_points.clone() {
                for n in np {
                    for p in &vns[n] {
                        norms.push(p.clone());
                    }
                }
            }
        } else {
            norms = create_normal(&vertices);
        }
        if g.name == "jacket" || g.name == "legs" || g.name == "skirt" {

        }
        let m = Model::init(vertices, uvs, norms, g.texture);
        models.push(m);

    }
    ModelGroup::new(models)
}

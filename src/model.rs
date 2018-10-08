use std::fs::File;
use std::io::prelude::*;
use math::Vector3;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Triangle {
    pub v0: Vector3,
    pub v1: Vector3,
    pub v2: Vector3,
    pub vt0: Vector3,
    pub vt1: Vector3,
    pub vt2: Vector3,
}

impl Triangle {
    pub fn new(v0: Vector3, v1: Vector3, v2: Vector3, vt0: Vector3, vt1: Vector3, vt2: Vector3) -> Triangle {
        Triangle { v0: v0,
                       v1: v1,
                       v2: v2,
                       vt0: vt0,
                       vt1: vt1,
                       vt2: vt2 }
    }
}

pub struct Model {
    vertices: Vec<Vector3>,
    uv: Vec<Vector3>,
    triangles: Vec<Triangle>
}

impl Model {
    pub fn new(filepath: &Path) -> Model {
        let mut f = File::open(filepath).expect(&format!("File not found: {:?}", filepath));
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).expect(&format!("Error reading file: {:?}", filepath));

        let mut vertices = Vec::new();
        let mut uv = Vec::new();
        let mut triangles = Vec::new();

        for line in file_contents.lines() {
            let t = line.split(" ").next().unwrap();

            match t {
                "v" => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let x = parts[1].parse::<f32>().expect(&format!("Could not parse vertex X value: {}", line));
                    let y = parts[2].parse::<f32>().expect(&format!("Could not parse vertex Y value: {}", line));
                    let z = parts[3].parse::<f32>().expect(&format!("Could not parse vertex Z value: {}", line));

                    vertices.push(Vector3::new(x, y, z));
                },
                "f" => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let t0: Vec<&str> = parts[1].split("/").collect();
                    let t1: Vec<&str> = parts[2].split("/").collect();
                    let t2: Vec<&str> = parts[3].split("/").collect();

                    let v0 = t0[0].parse::<usize>().expect(&format!("Could not parse vertex index: {}", line));
                    let v1 = t1[0].parse::<usize>().expect(&format!("Could not parse vertex index: {}", line));
                    let v2 = t2[0].parse::<usize>().expect(&format!("Could not parse vertex index: {}", line));

                    let vt0 = t0[1].parse::<usize>().expect(&format!("Could not parse texture vertex index: {}", line));
                    let vt1 = t1[1].parse::<usize>().expect(&format!("Could not parse texture vertex index: {}", line));
                    let vt2 = t2[1].parse::<usize>().expect(&format!("Could not parse texture vertex index: {}", line));

                    triangles.push(Triangle::new(vertices[v0-1], vertices[v1-1], vertices[v2-1], uv[vt0-1], uv[vt1-1], uv[vt2-1]));
                },
                "vt" => {
                    let parts: Vec<&str> = line.split(" ").filter(|x| x.len() > 0).collect();
                    let x = parts[1].parse::<f32>().expect(&format!("Could not parse texture vertex X value: {}", line));
                    let y = parts[2].parse::<f32>().expect(&format!("Could not parse texture vertex Y value: {}", line));
                    let z = parts[3].parse::<f32>().expect(&format!("Could not parse texture vertex Z value: {}", line));

                    uv.push(Vector3::new(x, y, z));
                }
                _ => ()
            }
        }

        Model { vertices: vertices, uv: uv, triangles: triangles }
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
}
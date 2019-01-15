use std::fs::File;
use std::io::prelude::*;
use tdmath::{Vector3, Vector4};
use std::path::Path;
use modelloader::*;

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub v0: Vector4,
    pub v1: Vector4,
    pub v2: Vector4,
    pub vt0: Vector3,
    pub vt1: Vector3,
    pub vt2: Vector3,
}

pub struct Model {
    triangles: Vec<Triangle>
}

impl Model {
    pub fn new(filepath: &Path) -> Model {
        let mut f = File::open(filepath).expect(&format!("File not found: {:?}", filepath));
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).expect(&format!("Error reading file: {:?}", filepath));

        let mut triangles = Vec::new();
        let m = parse_obj_file(&file_contents);
        for v in m.vertices.chunks(3) {
            let t = Triangle {
                v0: Vector4::new(v[0].p.x, v[0].p.y, v[0].p.z, 1.0),
                v1: Vector4::new(v[1].p.x, v[1].p.y, v[1].p.z, 1.0),
                v2: Vector4::new(v[2].p.x, v[2].p.y, v[2].p.z, 1.0),
                vt0: Vector3::new(v[0].uv.x, 1.0 - v[0].uv.y, 0.0),
                vt1: Vector3::new(v[1].uv.x, 1.0 - v[1].uv.y, 0.0),
                vt2: Vector3::new(v[2].uv.x, 1.0 - v[2].uv.y, 0.0),
            };

            triangles.push(t);
        }

        Model { triangles }
    }

    pub fn triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
}

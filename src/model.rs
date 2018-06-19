use std::fs::File;
use std::io::prelude::*;
use vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex { x: x, y: y, z: z}
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn position(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}

#[derive(Copy, Clone)]
struct Triangle {
    v0: usize,
    v1: usize,
    v2: usize,
}

impl Triangle {
    pub fn new(v0: usize, v1: usize, v2: usize) -> Triangle {
        Triangle{ v0: v0, v1: v1, v2: v2 }
    }
}

pub struct Model {
    vertices: Vec<Vertex>,
    triangles: Vec<Triangle>,
}

impl Model {
    pub fn new(filename: &str) -> Model {
        let mut f = File::open(filename).expect(&format!("File not found: {}", filename));
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).expect(&format!("Error reading file: {}", filename));

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for line in file_contents.lines() {
            let t = line.split(" ").next().unwrap();

            match t {
                "v" => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let x = parts[1].parse::<f32>().expect(&format!("Could not parse vertex X value: {}", line));
                    let y = parts[2].parse::<f32>().expect(&format!("Could not parse vertex Y value: {}", line));
                    let z = parts[3].parse::<f32>().expect(&format!("Could not parse vertex Z value: {}", line));

                    vertices.push(Vertex::new(x, y, z));
                },
                "f" => {
                    let parts: Vec<&str> = line.split(" ").collect();
                    let t0: Vec<&str> = parts[1].split("/").collect();
                    let t1: Vec<&str> = parts[2].split("/").collect();
                    let t2: Vec<&str> = parts[3].split("/").collect();

                    let v0 = t0[0].parse::<usize>().expect(&format!("Could not parse vertex index: {}", line));
                    let v1 = t1[0].parse::<usize>().expect(&format!("Could not parse vertex index: {}", line));
                    let v2 = t2[0].parse::<usize>().expect(&format!("Could not parse vertex index: {}", line));

                    triangles.push(Triangle::new(v0, v1, v2));
                },
                _ => ()
            }
        }
        
        Model { vertices: vertices, triangles: triangles }
    }

    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }

    pub fn get_triangle_vertices(&self, index: usize) -> (Vertex, Vertex, Vertex) {
        let t = self.triangles[index];

        (self.vertices[t.v0-1], self.vertices[t.v1-1], self.vertices[t.v2-1])
    }
}
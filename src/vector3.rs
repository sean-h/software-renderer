use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn dot(v0: Vector3, v1: Vector3) -> f32 {
        v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
    }

    pub fn cross(v0: Vector3, v1: Vector3) -> Vector3 {
        Vector3 {x: (v0.y * v1.z) - (v0.z * v1.y),
                 y: (v0.z * v1.x) - (v0.x * v1.z),
                 z: (v0.x * v1.y) - (v0.y * v1.x)}
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(&self) -> Vector3 {
        let l = self.length();
        Vector3::new(self.x / l, self.y / l, self.z / l)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x,
                 y: self.y + other.y,
                 z: self.z + other.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x,
                 y: self.y - other.y,
                 z: self.z - other.z}
    }
}
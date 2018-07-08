use std::ops::{Add, Sub, Mul};

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

    pub fn barycentric(point: Vector3, v0: Vector3, v1: Vector3, v2: Vector3) -> Option<Vector3> {
        let vec0 = v1 - v0;
        let vec1 = v2 - v0;
        let vec2 = point - v0;
        let d00 = Vector3::dot(vec0, vec0);
        let d01 = Vector3::dot(vec0, vec1);
        let d11 = Vector3::dot(vec1, vec1);
        let d20 = Vector3::dot(vec2, vec0);
        let d21 = Vector3::dot(vec2, vec1);
        let denom = d00 * d11 - d01 * d01;

        let v = (d11 * d20 - d01 * d21) / denom;
        let w = (d00 * d21 - d01 * d20) / denom;
        let u = 1.0 - v - w;

        Some(Vector3::new(u, v, w))
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

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Vector3 {
        Vector3 {x: self.x * other,
                 y: self.y * other,
                 z: self.z * other}
    }
}

#[cfg(test)]
mod tests {
    use vector3::*;

    #[test]
    fn test_vector_subtraction() {
        let a = Vector3 {x: 1.0, y: 1.0, z: 1.0};
        let b = Vector3 {x: 1.0, y: 2.0, z: 3.0};
        let c = a - b;
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, -1.0);
        assert_eq!(c.z, -2.0);

        let d = a - b;
        assert_eq!(d.x, 0.0);
    }
}
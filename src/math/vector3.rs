use std::ops::{Add, Sub, Mul, Neg};

/// A 3 axis vector of `f32` values.
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    /// Returns a new `Vector3`.
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x: x, y: y, z: z }
    }

    /// Returns a `Vector3` with a length of 0.
    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    /// Returns a `Vector3` of values (0, 1, 0).
    pub fn up() -> Vector3 {
        Vector3 { x: 0.0, y: 1.0, z: 0.0 }
    }

    /// Returns a `Vector3` of values (0, 0, 1).
    pub fn forward() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 1.0 }
    }

    /// Returns a `Vector3` of values (1, 0, 0).
    pub fn left() -> Vector3 {
        Vector3 { x: 1.0, y: 0.0, z: 0.0 }
    }

    /// Returns the dot product of `v0` and `v1`.
    pub fn dot(v0: Vector3, v1: Vector3) -> f32 {
        v0.x * v1.x + v0.y * v1.y + v0.z * v1.z
    }

    /// Returns the cross product of `v0` and `v1`.
    pub fn cross(v0: Vector3, v1: Vector3) -> Vector3 {
        Vector3 {x: (v0.y * v1.z) - (v0.z * v1.y),
                 y: (v0.z * v1.x) - (v0.x * v1.z),
                 z: (v0.x * v1.y) - (v0.y * v1.x)}
    }

    /// Returns the length of the vector before taking the square root.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a new normalized `Vector3` of the vector.
    pub fn normalized(&self) -> Vector3 {
        let l = self.length();
        Vector3::new(self.x / l, self.y / l, self.z / l)
    }

    /// Returns the barycentric coordinates for `point` inside a triangle defined by `v0`, `v1`, `v2`.
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

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {x: -self.x, y: -self.y, z: -self.z}
    }
}

#[cfg(test)]
mod tests {
    use math::Vector3;

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

    #[test]
    fn test_vector_cross() {
        let x = Vector3 {x: 1.0, y: 0.0, z: 0.0};
        let y = Vector3 {x: 0.0, y: 1.0, z: 0.0};
        let z = Vector3::cross(x, y);

        assert_eq!(0.0, z.x);
        assert_eq!(0.0, z.y);
        assert_eq!(1.0, z.z);
    }
}
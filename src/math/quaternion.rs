use std::f32;
use std::ops::{Mul};
use math::Vector3;

/// A quaternion of `f32` values.
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    /// Returns a new `Quaternion` with a rotation of x, y, z in radians.
    pub fn new(x: f32, y: f32, z: f32) -> Quaternion {
        let x2 = x / 2.0;
        let y2 = y / 2.0;
        let z2 = z / 2.0;

        let x2_cos = x2.cos();
        let y2_cos = y2.cos();
        let z2_cos = z2.cos();

        let x2_sin = x2.sin();
        let y2_sin = y2.sin();
        let z2_sin = z2.sin();

        Quaternion {
            x: x2_sin * y2_cos * z2_cos - x2_cos * y2_sin * z2_sin,
            y: x2_cos * y2_sin * z2_cos + x2_sin * y2_cos * z2_sin,
            z: x2_cos * y2_cos * z2_sin - x2_sin * y2_sin * z2_cos,
            w: x2_cos * y2_cos * z2_cos + x2_sin * y2_sin * z2_sin,
        }
    }

    /// Returns an identity quaternion.
    pub fn identity() -> Quaternion {
        Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, other: Quaternion) -> Quaternion {
        let qv = Vector3::new(self.x, self.y, self.z);
        let rv = Vector3::new(other.x, other.y, other.z);

        let v = Vector3::cross(qv, rv) + (qv * other.w) + (rv * self.w);
        let w = (self.w * other.w) - Vector3::dot(qv, rv);

        Quaternion {
            x: v.x,
            y: v.y,
            z: v.z,
            w: w,
        }
    }
}

#[cfg(test)]
mod tests {
    use math::Quaternion;
    use std::f32::consts::FRAC_PI_2;

    #[test]
    fn test_quaternion_new() {
        let q = Quaternion::new(0.0, FRAC_PI_2, 0.0);
        assert!((q.y - 0.7071).abs() < 0.001);
        assert!((q.w - 0.7071).abs() < 0.001);
    }
}
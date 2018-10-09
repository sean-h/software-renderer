use std::ops::{Mul, Index, IndexMut};
use math::{Vector3, Quaternion};

/// A 4x4 matrix of `f32` values.
#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    data: [[f32; 4]; 4]
}

impl Matrix4 {
    pub fn zero() -> Matrix4 {
        Matrix4 {data: [[0.0, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0],
                        [0.0, 0.0, 0.0, 0.0]]}
    }

    /// Returns an identity matrix.
    pub fn identity() -> Matrix4 {
        Matrix4 {data: [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]]}
    }

    /// Returns a translation matrix for translations along the `x`, `y` and `z` axes.
    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {data: [[1.0, 0.0, 0.0,   x],
                        [0.0, 1.0, 0.0,   y],
                        [0.0, 0.0, 1.0,   z],
                        [0.0, 0.0, 0.0, 1.0]]}
    }

    /// Returns a scale matrix that scales along the `x`, `y` and `z` axes.
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {data: [[  x, 0.0, 0.0, 0.0],
                        [0.0,   y, 0.0, 0.0],
                        [0.0, 0.0,   z, 0.0],
                        [0.0, 0.0, 0.0, 1.0]]}
    }

    /// Returns a rotation matrix for a `Quaternion`.
    pub fn rotation(q: Quaternion) -> Matrix4 {
        let qx2 = q.x * q.x;
        let qy2 = q.y * q.y;
        let qz2 = q.z * q.z;
        let qxqy = q.x * q.y;
        let qxqz = q.x * q.z;
        let qyqz = q.y * q.z;
        let qwqx = q.w * q.x;
        let qwqy = q.w * q.y;
        let qwqz = q.w * q.z;

        Matrix4 {data: [[1.0 - 2.0 * (qy2 + qz2),     2.0 * (qxqy - qwqz),     2.0 * (qxqz + qwqy), 0.0],
                        [    2.0 * (qxqy + qwqz), 1.0 - 2.0 * (qx2 + qz2),     2.0 * (qyqz - qwqx), 0.0],
                        [    2.0 * (qxqz - qwqy),     2.0 * (qyqz + qwqx), 1.0 - 2.0 * (qx2 + qy2), 0.0],
                        [                    0.0,                     0.0,                     0.0, 1.0]]}
    }

    /// Returns a look at matrix that looks from `position` to `look`.
    pub fn look_at(position: Vector3, look: Vector3, up: Vector3) -> Matrix4 {
        // forward points away from the look direction
        let forward = (position - look).normalized();
        let left = Vector3::cross(forward, up.normalized()).normalized();
        let new_up = Vector3::cross(forward, left);

        Matrix4 {data: [[left.x, new_up.x, forward.x, -Vector3::dot(left, position)],
                        [left.y, new_up.y, forward.y, -Vector3::dot(new_up, position)],
                        [left.z, new_up.z, forward.z, -Vector3::dot(forward, position)],
                        [    0.0,     0.0,       0.0,                              1.0]]}
    }

    /// Returns an orthographic projection matrix.
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4 {
        Matrix4 {data: [[2.0 / (right - left),                  0.0,                  0.0, -(right + left) / (right - left)],
                        [                 0.0, 2.0 / (top - bottom),                  0.0, -(top + bottom) / (top - bottom)],
                        [                 0.0,                  0.0, 2.0 / (-far - -near), -(-far + -near) / (-far - -near)],
                        [                 0.0,                  0.0,                  0.0,                              1.0]]}
    }

    pub fn perpective(fov: f32, near: f32, far: f32) -> Matrix4 {
        let mut m = Matrix4::zero();
        let s = 1.0 / ((fov / 2.0 * 3.14159 / 180.0)).tan();

        m[0][0] = s;
        m[1][1] = s;
        m[2][2] = -far / (far - near);
        m[2][3] = -far * near / (far - near);
        m[3][2] = -1.0;

        m
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f32];

    fn index(&self, index: usize) -> &[f32] {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut [f32] {
        &mut self.data[index]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut r = Matrix4::identity();
        for i in 0..4 {
            for j in 0..4 {
                r[i][j] = self[i][0] * other[0][j] +
                          self[i][1] * other[1][j] +
                          self[i][2] * other[2][j] +
                          self[i][3] * other[3][j];
            }
        }
        return r;
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        let x = other.x * self[0][0] + other.y * self[0][1] + other.z * self[0][2] + self[0][3];
        let y = other.x * self[1][0] + other.y * self[1][1] + other.z * self[1][2] + self[1][3];
        let z = other.x * self[2][0] + other.y * self[2][1] + other.z * self[2][2] + self[2][3];

        Vector3::new(x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use math::Vector3;
    use math::Matrix4;

    #[test]
    fn test_vector_scale_matrix_multiplication() {
        let v = Vector3 {x: 4.0, y: 1.0, z: 8.0};
        let m = Matrix4::scale(0.5, 0.5, 0.5);

        let mv = m * v;
        assert_eq!(mv.x, 2.0);
        assert_eq!(mv.y, 0.5);
        assert_eq!(mv.z, 4.0);
    }

    #[test]
    fn test_vector_translation_matrix_multiplication() {
        let v = Vector3 {x: 4.0, y: 1.0, z: 8.0};
        let m = Matrix4::translation(0.0, 3.0, -5.0);

        let mv = m * v;
        assert_eq!(mv.x, 4.0);
        assert_eq!(mv.y, 4.0);
        assert_eq!(mv.z, 3.0);
    }

    #[test]
    fn test_matrix_look_at() {
        let pos = Vector3::new(2.0, -5.0, 0.0);
        let look = Vector3::new(3.0, -5.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);

        let m = Matrix4::look_at(pos, look, up);
        let v = Vector3::new(4.0, -5.0, 0.0);
        let mv = m * v;

        assert_eq!(mv.x, 0.0);
        assert_eq!(mv.y, 0.0);
        assert_eq!(mv.z, -2.0);
    }
}
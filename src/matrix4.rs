use std::ops::{Mul, Index, IndexMut};
use vector3::Vector3;
use quaternion::Quaternion;

#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    data: [[f32; 4]; 4]
}

impl Matrix4 {
    pub fn identity() -> Matrix4 {
        Matrix4 {data: [[1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0]]}
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {data: [[1.0, 0.0, 0.0,   x],
                        [0.0, 1.0, 0.0,   y],
                        [0.0, 0.0, 1.0,   z],
                        [0.0, 0.0, 0.0, 1.0]]}
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4 {
        Matrix4 {data: [[  x, 0.0, 0.0, 0.0],
                        [0.0,   y, 0.0, 0.0],
                        [0.0, 0.0,   z, 0.0],
                        [0.0, 0.0, 0.0, 1.0]]}
    }

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
    use vector3::Vector3;
    use matrix4::Matrix4;

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
}
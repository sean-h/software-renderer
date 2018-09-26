use vector3::Vector3;
use quaternion::Quaternion;

pub enum Projection {
    Orthographic,
    Perspective(f32),
}

pub struct Camera {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub projection: Projection,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 1.0),
            projection: Projection::Orthographic,
        }
    }
}
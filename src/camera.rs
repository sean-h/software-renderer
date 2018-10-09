use math::{Vector3, Quaternion};

pub enum Projection {
    Orthographic(f32),
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
            position: Vector3::new(0.0, 0.0, -10.0),
            rotation: Quaternion::new(0.0, 0.0, 1.0),
            projection: Projection::Orthographic(1.0),
        }
    }
}
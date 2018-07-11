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


//! 3D Camera

extern crate tdmath;

use self::tdmath::{Vector3, Quaternion};

/// 3D Projections
pub enum Projection {
    /// Orthographic projection with scale
    Orthographic(f32),

    /// Perspective projection with FoV
    Perspective(f32),
}

/// 3D Camera with position, rotation and projection.
pub struct Camera {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub projection: Projection,
}

impl Camera {
    /// Returns a new `Camera` at the origin with a Perspective projection with a FoV of 60.
    pub fn new() -> Camera {
        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0),
            projection: Projection::Perspective(60.0),
        }
    }
}
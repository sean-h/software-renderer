use vector3::*;
use std::cmp::{min, max};

#[derive(Debug, Copy, Clone)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Vector2i {
        Vector2i { x: x, y: y }
    }

    pub fn barycentric(point: Vector2i, v0: Vector2i, v1: Vector2i, v2: Vector2i) -> Option<Vector3> {
        let x = Vector3::new((v2.x - v0.x) as f32, (v1.x - v0.x) as f32, (v0.x - point.x) as f32);
        let y = Vector3::new((v2.y - v0.y) as f32, (v1.y - v0.y) as f32, (v0.y - point.y) as f32);

        let u = Vector3::cross(x, y);

        if u.z < 1.0 {
            return None;
        }

        Some(Vector3::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z))
    }

    pub fn bbox3(v0: Vector2i, v1: Vector2i, v2: Vector2i) -> (Vector2i, Vector2i) {
        let min_x = min(v0.x, min(v1.x, v2.x));
        let max_x = max(v0.x, max(v1.x, v2.x));
        let min_y = min(v0.y, min(v1.y, v2.y));
        let max_y = max(v0.y, max(v1.y, v2.y));

        (Vector2i::new(min_x, min_y),
         Vector2i::new(max_x, max_y))
    }
}
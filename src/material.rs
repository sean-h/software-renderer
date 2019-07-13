//! Material

extern crate image;

use image::DynamicImage;
use std::collections::HashMap;

/// Material
pub struct Material {
    pub albedo: Option<Box<DynamicImage>>,
}

impl Material {
    /// Returns a new `Material` from a map of texture types and texture paths.
    pub fn from_hashmap(material_map: HashMap<String, String>) -> Material {
        let albedo = match material_map.get("albedo") {
            Some(albedo_path) => Some(Box::new(image::open(albedo_path).expect(&format!("Cannot load texture: {}", albedo_path)))),
            None => None,
        };

        Material {
            albedo,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Material {
            albedo: None,
        }
    }
}
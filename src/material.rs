extern crate image;

use image::{GenericImage, DynamicImage};
use std::collections::HashMap;

pub struct Material {
    pub albedo: Option<Box<DynamicImage>>,
    specular: Option<Box<DynamicImage>>,
    normal: Option<Box<DynamicImage>>,
}

impl Material {
    pub fn new() -> Material {
        Material {
            albedo: None,
            specular: None,
            normal: None,
        }
    }

    pub fn from_hashmap(material_map: HashMap<String, String>) -> Material {
        let albedo = match material_map.get("albedo") {
            Some(albedo_path) => Some(Box::new(image::open(albedo_path).expect(&format!("Cannot load texture: {}", albedo_path)))),
            None => None,
        };

        Material {
            albedo: albedo,
            specular: None,
            normal: None,
        }
    }
}
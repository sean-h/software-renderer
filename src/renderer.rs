extern crate sdl2;
extern crate tdmath;

use self::tdmath::{Vector3, Vector2i, Matrix4};
use model::{Model, Triangle};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use std::mem;
use zbuffer::ZBuffer;
use image::{GenericImage, DynamicImage};
use camera::{Camera, Projection};
use std::path::{Path, PathBuf};
use material::Material;
use std::fs::File;
use std::io::prelude::*;
use toml::Value;
use std::collections::HashMap;

pub struct Renderer {
    light_dir: Vector3,
    models: Vec<Model>,
    zbuffer: ZBuffer,
    material: Material,
    camera: Camera,
    rot_x: f32,
    ambient_intensity: f32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { light_dir: Vector3::new(0.0, 0.0, 1.0),
                   models: Vec::new(),
                   zbuffer: ZBuffer::new(800, 800),
                   material: Material::new(),
                   camera: Camera::new(),
                   rot_x: 1.57,
                   ambient_intensity: 0.0 }
    }

    pub fn load_models(&mut self, model_paths: Vec<&Path>) {
        for path in model_paths {
            let model = Model::new(path);
            self.models.push(model);
        }
    }

    pub fn load_material(&mut self, material_path: &Path) {
        let mut f = File::open(material_path.clone()).expect(&format!("File not found: {:?}", material_path));
        let mut file_contents = String::new();
        f.read_to_string(&mut file_contents).expect(&format!("Error reading file: {:?}", material_path));

        let toml = file_contents.parse::<Value>().expect(&format!("Unable to parse material: {:?}", material_path));

        let mut material_map = HashMap::new();

        match toml.get("albedo") {
            Some(albedo) => {
                if let Some(albedo_path) = albedo.as_str() {
                    let mut full_path = PathBuf::new();
                    full_path.push(material_path);
                    full_path.pop(); // Remove material name
                    full_path.push(albedo_path);
                    material_map.insert("albedo".to_owned(), full_path.to_str().unwrap().to_owned());
                }
            },
            None => ()
        }

        match toml.get("specular") {
            Some(specular) => {
                if let Some(specular_path) = specular.as_str() {
                    let mut full_path = PathBuf::new();
                    full_path.push(material_path);
                    full_path.pop(); // Remove material name
                    full_path.push(specular_path);
                    material_map.insert("specular".to_owned(), full_path.to_str().unwrap().to_owned());
                }
            },
            None => ()
        }

        match toml.get("normal") {
            Some(normal) => {
                if let Some(normal_path) = normal.as_str() {
                    let mut full_path = PathBuf::new();
                    full_path.push(material_path);
                    full_path.pop(); // Remove material name
                    full_path.push(normal_path);
                    material_map.insert("normal".to_owned(), full_path.to_str().unwrap().to_owned());
                }
            },
            None => ()
        }

        self.material = Material::from_hashmap(material_map);
    }

    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.zbuffer.clear();

        let (width, height) = canvas.viewport().size();
        let aspect = width as f32 / height as f32;

        let projection = match self.camera.projection {
            Projection::Orthographic(scale) => Matrix4::ortho(scale * aspect, -scale * aspect, -scale, scale, 0.1, 50.0),
            Projection::Perspective(fov) => Matrix4::perpective(fov, -aspect, 0.1, 50.0)
        };

        for model in &self.models {
            let mvp = projection
            * Matrix4::look_at(self.camera.position, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0))
            * Matrix4::translation(0.0, 0.0, 0.0)
            * Matrix4::scale(1.0, 1.0, 1.0);

            for triangle in model.triangles() {
                let mut triangle_mvp = *triangle;

                triangle_mvp.v0 = mvp * triangle_mvp.v0;
                triangle_mvp.v1 = mvp * triangle_mvp.v1;
                triangle_mvp.v2 = mvp * triangle_mvp.v2;

                let normal = Vector3::cross(triangle.v2.xyz() - triangle.v0.xyz(), triangle.v1.xyz() - triangle.v0.xyz()).normalized();

                let camera_forward = (self.camera.position).normalized();
                if Vector3::dot(normal, camera_forward) > 0.0 {
                    continue;
                }

                let intensity = Vector3::dot(normal, self.light_dir);
                let intensity = if intensity < 0.0 {
                    -intensity
                } else {
                    self.ambient_intensity
                };

                Renderer::draw_triangle(canvas, triangle_mvp, &mut self.zbuffer, &self.material.albedo, intensity);
            }
        }
    }

    fn draw_triangle(canvas: &mut Canvas<sdl2::video::Window>, triangle: Triangle, zbuffer: &mut ZBuffer, texture: &Option<Box<DynamicImage>>, intensity: f32) {
        let canvas_width = canvas.viewport().width() as f32;
        let canvas_height = canvas.viewport().height() as f32;

        let ndc0 = Vector3::new(triangle.v0.x / triangle.v0.w, triangle.v0.y / triangle.v0.w, triangle.v0.z / triangle.v0.w);
        let ndc1 = Vector3::new(triangle.v1.x / triangle.v1.w, triangle.v1.y / triangle.v1.w, triangle.v1.z / triangle.v1.w);
        let ndc2 = Vector3::new(triangle.v2.x / triangle.v2.w, triangle.v2.y / triangle.v2.w, triangle.v2.z / triangle.v2.w);

        let p0 = Vector2i::new(to_screen_space(ndc0.x, canvas_width as f32), to_screen_space(ndc0.y, canvas_height as f32));
        let p1 = Vector2i::new(to_screen_space(ndc1.x, canvas_width as f32), to_screen_space(ndc1.y, canvas_height as f32));
        let p2 = Vector2i::new(to_screen_space(ndc2.x, canvas_width as f32), to_screen_space(ndc2.y, canvas_height as f32));
        
        let screen_space0 = Vector3::new((ndc0.x + 1.0) * canvas_width / 2.0, (ndc0.y + 1.0) * canvas_height / 2.0, ndc0.z);
        let screen_space1 = Vector3::new((ndc1.x + 1.0) * canvas_width / 2.0, (ndc1.y + 1.0) * canvas_height / 2.0, ndc1.z);
        let screen_space2 = Vector3::new((ndc2.x + 1.0) * canvas_width / 2.0, (ndc2.y + 1.0) * canvas_height / 2.0, ndc2.z);

        let (bbox_min, bbox_max) = Vector2i::bbox3(p0, p1, p2);

        for x in bbox_min.x..bbox_max.x+1 {
            for y in bbox_min.y..bbox_max.y+1 {
                if x >= canvas_width as i32 || y >= canvas_height as i32 || x < 0 || y < 0  {
                    continue;
                }

                let b = Vector3::barycentric(Vector3::new(x as f32, y as f32, 0.0), screen_space0, screen_space1, screen_space2);

                match b {
                    Some(uvw) => {
                        if uvw.x >= 0.0 && uvw.y >= 0.0 && uvw.z >= 0.0 {
                            let z_distance = uvw.x * screen_space0.z + uvw.y * screen_space1.z + uvw.z * screen_space2.z;

                            if z_distance.abs() <= 1.0 && z_distance < zbuffer.sample(x as usize, y as usize) {
                                zbuffer.set(z_distance, x as usize, y as usize);

                                let mut clip = Vector3::new(uvw.x / triangle.v0.w, uvw.y / triangle.v1.w, uvw.z / triangle.v2.w);
                                clip = clip / (clip.x + clip.y + clip.z);

                                let u = clip.x * triangle.vt0.x + clip.y * triangle.vt1.x + clip.z * triangle.vt2.x;
                                let v = clip.x * triangle.vt0.y + clip.y * triangle.vt1.y + clip.z * triangle.vt2.y;

                                let (r, g, b) = match texture {
                                    Some(texture) => {
                                        let w = texture.width() as f32;
                                        let h = texture.height() as f32;

                                        let x = clamp((u * w) as u32, 0, w as u32 - 1);
                                        let y = clamp((v * h) as u32, 0, h as u32 - 1);

                                        let color = texture.get_pixel(x, y);

                                        ((color.data[0] as f32 * intensity) as u8, (color.data[1] as f32 * intensity) as u8, (color.data[2] as f32 * intensity) as u8)
                                    },
                                    None => ((128.0 * intensity) as u8, (128.0 * intensity) as u8, (128.0 * intensity) as u8)
                                };
                                
                                canvas.set_draw_color(Color::RGB(r, g, b));
                                canvas.draw_point(Point::new(x, y)).unwrap();
                            }
                        }
                    },
                    None => ()
                }
            }
        }
    }

    pub fn zoom_camera(&mut self, zoom_amount: f32) {
        match self.camera.projection {
            Projection::Orthographic(scale) => {
                self.camera.projection = Projection::Orthographic(scale + zoom_amount * 0.1)
            },
            Projection::Perspective(fov) => {
                self.camera.projection = Projection::Perspective(fov + zoom_amount)
            }
        }
    }

    pub fn toggle_projection_mode(&mut self) {
        self.camera.projection = match self.camera.projection {
            Projection::Orthographic(_) => Projection::Perspective(60.0),
            Projection::Perspective(_) => Projection::Orthographic(5.0),
        }
    }

    fn draw_line(canvas: &mut Canvas<sdl2::video::Window>, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut steep = false;

        let mut x0 = x0;
        let mut x1 = x1;
        let mut y0 = y0;
        let mut y1 = y1;

        if (x0 - x1).abs() < (y0 - y1).abs()
        {
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
            steep = true;
        }

        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let d_error_2 = dy.abs() * 2;
        let mut error_2 = 0;
        let mut y = y0;

        for x in x0..x1+1 {
            if steep {
                canvas.draw_point(Point::new(y, x)).unwrap();
            } else {
                canvas.draw_point(Point::new(x, y)).unwrap();
            }

            error_2 += d_error_2;
            if error_2 > dx {
                y += if y1 > y0 {
                    1
                } else {
                    -1
                };

                error_2 -= dx * 2;
            }
        }
    }

    pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        self.rot_x += delta_x;
        self.camera.position = Vector3::new(10.0 * self.rot_x.cos(), 0.0, 10.0 * self.rot_x.sin());
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.zbuffer.resize(width, height);
    }

    pub fn increase_ambient_intensity(&mut self, delta: f32) {
        self.ambient_intensity += delta;
    }
}

fn to_screen_space(num: f32, dimension: f32) -> i32 {
    ((num + 1.0) * dimension / 2.0) as i32
}

fn clamp(val: u32, min: u32, max: u32) -> u32 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
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
use std::path::Path;
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
    timer: f32,
    camera: Camera,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { light_dir: Vector3::new(0.0, 0.0, 1.0),
                   models: Vec::new(),
                   zbuffer: ZBuffer::new(800, 800),
                   material: Material::new(),
                   timer: 0.0,
                   camera: Camera::new() }
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
                    material_map.insert("albedo".to_owned(), albedo_path.to_owned());
                }
            },
            None => ()
        }

        match toml.get("specular") {
            Some(specular) => {
                if let Some(specular_path) = specular.as_str() {
                    material_map.insert("specular".to_owned(), specular_path.to_owned());
                }
            },
            None => ()
        }

        match toml.get("normal") {
            Some(normal) => {
                if let Some(normal_path) = normal.as_str() {
                    material_map.insert("normal".to_owned(), normal_path.to_owned());
                }
            },
            None => ()
        }

        self.material = Material::from_hashmap(material_map);
    }

    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.zbuffer.clear();

        self.timer += 0.1;
        self.camera.position = Vector3::new(10.0 * self.timer.cos(), 0.0, 10.0 * self.timer.sin());

        let projection = match self.camera.projection {
            Projection::Orthographic(scale) => Matrix4::ortho(-scale, scale, -scale, scale, 0.1, 50.0),
            Projection::Perspective(fov) => Matrix4::perpective(fov, 0.1, 50.0)
        };

        for model in &self.models {
            let mvp = projection
            * Matrix4::look_at(self.camera.position, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0))
            * Matrix4::translation(0.0, 0.0, 0.0)
            * Matrix4::scale(1.0, 1.0, 1.0);

            for triangle in model.triangles() {
                let mut triangle_mvp = *triangle;

                let v0 = mvp * triangle_mvp.v0.to_vector4(1.0);
                let v1 = mvp * triangle_mvp.v1.to_vector4(1.0);
                let v2 = mvp * triangle_mvp.v2.to_vector4(1.0);

                triangle_mvp.v0 = Vector3::new(v0.x / v0.w, v0.y / v0.w, v0.z / v0.w);
                triangle_mvp.v1 = Vector3::new(v1.x / v1.w, v1.y / v1.w, v1.z / v1.w);
                triangle_mvp.v2 = Vector3::new(v2.x / v2.w, v2.y / v2.w, v2.z / v2.w);

                let normal = Vector3::cross(triangle.v2 - triangle.v0, triangle.v1 - triangle.v0).normalized();

                let camera_forward = (self.camera.position).normalized();
                if Vector3::dot(normal, camera_forward) > 0.0 {
                    continue;
                }

                let intensity = Vector3::dot(normal, self.light_dir);
                let intensity = if intensity < 0.0 {
                    -intensity
                } else {
                    0.0
                };

                Renderer::draw_triangle(canvas, triangle_mvp, &mut self.zbuffer, &self.material.albedo, intensity);
            }
        }
    }

    fn draw_triangle(canvas: &mut Canvas<sdl2::video::Window>, triangle: Triangle, zbuffer: &mut ZBuffer, texture: &Option<Box<DynamicImage>>, intensity: f32) {
        let canvas_width = canvas.viewport().width() as f32;
        let canvas_height = canvas.viewport().height() as f32;

        let p0 = Vector2i::new(to_screen_space(triangle.v0.x, canvas_width as f32), to_screen_space(triangle.v0.y, canvas_height as f32));
        let p1 = Vector2i::new(to_screen_space(triangle.v1.x, canvas_width as f32), to_screen_space(triangle.v1.y, canvas_height as f32));
        let p2 = Vector2i::new(to_screen_space(triangle.v2.x, canvas_width as f32), to_screen_space(triangle.v2.y, canvas_height as f32));
        
        let screen_space0 = Vector3::new((triangle.v0.x + 1.0) * canvas_width / 2.0, (triangle.v0.y + 1.0) * canvas_height / 2.0, triangle.v0.z);
        let screen_space1 = Vector3::new((triangle.v1.x + 1.0) * canvas_width / 2.0, (triangle.v1.y + 1.0) * canvas_height / 2.0, triangle.v1.z);
        let screen_space2 = Vector3::new((triangle.v2.x + 1.0) * canvas_width / 2.0, (triangle.v2.y + 1.0) * canvas_height / 2.0, triangle.v2.z);

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

                                let u = uvw.x * triangle.vt0.x + uvw.y * triangle.vt1.x + uvw.z * triangle.vt2.x;
                                let v = 1.0 - (uvw.x * triangle.vt0.y + uvw.y * triangle.vt1.y + uvw.z * triangle.vt2.y);

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
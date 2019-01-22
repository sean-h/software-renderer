//! Software Renderer

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

/// Renderer
pub struct Renderer {
    light_dir: Vector3,
    models: Vec<Model>,
    zbuffer: ZBuffer,
    material: Material,
    camera: Camera,
    rot_x: f32,
    ambient_intensity: f32,
    smooth_shading: bool,
}

impl Renderer {
    /// Returns a new `Renderer`.
    pub fn new(width: usize, height: usize) -> Renderer {
        Renderer { light_dir: Vector3::new(0.0, 0.0, -1.0),
                   models: Vec::new(),
                   zbuffer: ZBuffer::new(width, height),
                   material: Material::new(),
                   camera: Camera::new(),
                   rot_x: 1.57,
                   ambient_intensity: 0.0,
                   smooth_shading: true,
        }
    }

    /// Loads the models in `model_paths`.
    pub fn load_models(&mut self, model_paths: Vec<&Path>) {
        for path in model_paths {
            let model = Model::new(path);
            self.models.push(model);
        }
    }

    /// Load the material at the `material_path`.
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

    /// Render the scene to the canvas.
    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.zbuffer.clear();

        let (width, height) = canvas.viewport().size();
        let aspect = width as f32 / height as f32;

        let projection = match self.camera.projection {
            Projection::Orthographic(scale) => Matrix4::ortho(scale * aspect, -scale * aspect, -scale, scale, 0.1, 50.0),
            Projection::Perspective(fov) => Matrix4::perpective(fov, -aspect, 0.1, 50.0)
        };

        let view = Matrix4::look_at(self.camera.position, Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));

        for model in &self.models {
            let model_matrix = Matrix4::translation(0.0, 0.0, 0.0) * Matrix4::scale(1.0, 1.0, 1.0);

            let render_params = RenderParameters {
                model: model_matrix,
                view: view,
                projection: projection,
                light_dir: self.light_dir,
                texture: &self.material.albedo,
                ambient_intensity: self.ambient_intensity,
                smooth_shading: self.smooth_shading,
            };

            for triangle in model.triangles() {
                let normal = Vector3::cross(triangle.v2.xyz() - triangle.v0.xyz(), triangle.v1.xyz() - triangle.v0.xyz()).normalized();

                let camera_forward = (self.camera.position).normalized();
                if Vector3::dot(normal, camera_forward) > 0.0 {
                    continue;
                }

                Renderer::draw_triangle(canvas, &mut self.zbuffer, *triangle, &render_params);
            }
        }
    }

    /// Draw a triangle to the canvas.
    fn draw_triangle(canvas: &mut Canvas<sdl2::video::Window>, zbuffer: &mut ZBuffer, triangle: Triangle, render_params: &RenderParameters) {
        let canvas_width = canvas.viewport().width() as f32;
        let canvas_height = canvas.viewport().height() as f32;

        let v0mvp = render_params.projection * render_params.view * render_params.model * triangle.v0;
        let v1mvp = render_params.projection * render_params.view * render_params.model * triangle.v1;
        let v2mvp = render_params.projection * render_params.view * render_params.model * triangle.v2;

        let ndc0 = Vector3::new(v0mvp.x / v0mvp.w, v0mvp.y / v0mvp.w, v0mvp.z / v0mvp.w);
        let ndc1 = Vector3::new(v1mvp.x / v1mvp.w, v1mvp.y / v1mvp.w, v1mvp.z / v1mvp.w);
        let ndc2 = Vector3::new(v2mvp.x / v2mvp.w, v2mvp.y / v2mvp.w, v2mvp.z / v2mvp.w);

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

                                let mut clip = Vector3::new(uvw.x / v0mvp.w, uvw.y / v1mvp.w, uvw.z / v2mvp.w);
                                clip = clip / (clip.x + clip.y + clip.z);

                                let u = clip.x * triangle.vt0.x + clip.y * triangle.vt1.x + clip.z * triangle.vt2.x;
                                let v = clip.x * triangle.vt0.y + clip.y * triangle.vt1.y + clip.z * triangle.vt2.y;

                                let normal = if render_params.smooth_shading {
                                    let n0 = clip.x * triangle.vn0.x + clip.y * triangle.vn1.x + clip.z * triangle.vn2.x;
                                    let n1 = clip.x * triangle.vn0.y + clip.y * triangle.vn1.y + clip.z * triangle.vn2.y;
                                    let n2 = clip.x * triangle.vn0.z + clip.y * triangle.vn1.z + clip.z * triangle.vn2.z;
                                    Vector3::new(n0, n1, n2)
                                } else {
                                    Vector3::cross(triangle.v1.xyz() - triangle.v0.xyz(), triangle.v2.xyz() - triangle.v0.xyz()).normalized()
                                };

                                let intensity = -Vector3::dot(normal, render_params.light_dir);
                                let intensity = if intensity >= 0.0 {
                                    intensity
                                } else {
                                    render_params.ambient_intensity
                                };

                                let (r, g, b) = match render_params.texture {
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

    /// Zoom by camera by `zoom_amount`.
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

    /// Toggles the camera's projection mode.
    pub fn toggle_projection_mode(&mut self) {
        self.camera.projection = match self.camera.projection {
            Projection::Orthographic(_) => Projection::Perspective(60.0),
            Projection::Perspective(_) => Projection::Orthographic(5.0),
        }
    }

    /// Draw a line on the canvas.
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

    /// Orbit the camera around the origin.
    pub fn orbit(&mut self, delta_x: f32, delta_y: f32) {
        self.rot_x += delta_x;
        self.camera.position = Vector3::new(10.0 * self.rot_x.cos(), 0.0, 10.0 * self.rot_x.sin());
    }

    /// Resize the render window. Call this to resize the `ZBuffer`.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.zbuffer.resize(width, height);
    }

    /// Increase the scene's ambient intensity.
    pub fn increase_ambient_intensity(&mut self, delta: f32) {
        self.ambient_intensity = clamp(self.ambient_intensity + delta, 0.0, 1.0);
    }

    /// Returns the text representation of the current projection mode.
    pub fn projection_mode_str(&self) -> &str {
        match self.camera.projection {
            Projection::Perspective(_) => "Perspective",
            Projection::Orthographic(_) => "Orthographic",
        }
    }

    /// Toggle the smooth shading option.
    pub fn toggle_smooth_shading(&mut self) {
        self.smooth_shading = !self.smooth_shading;
    }

    /// Returns the text representation of the current smooth shading option.
    pub fn smooth_shading_str(&self) -> &str {
        match self.smooth_shading {
            true => "Enabled",
            false => "Disabled"
        }
    }
}

/// Coverts a scalar value to the screen space position.
fn to_screen_space(num: f32, dimension: f32) -> i32 {
    ((num + 1.0) * dimension / 2.0) as i32
}

/// Clamps `val` between `min` and `max`.
fn clamp<T>(val: T, min: T, max: T) -> T
where T: PartialOrd
{
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

/// Parameters to pass into the triangle renderer.
pub struct RenderParameters<'a> {
    pub model: Matrix4,
    pub view: Matrix4,
    pub projection: Matrix4,
    pub light_dir: Vector3,
    pub texture: &'a Option<Box<DynamicImage>>,
    pub ambient_intensity: f32,
    pub smooth_shading: bool,
}
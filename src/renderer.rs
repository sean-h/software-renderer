extern crate sdl2;

use vector3::Vector3;
use vector2::Vector2i;
use model::{Model, Triangle};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use std::mem;
use zbuffer::ZBuffer;
use image;
use image::{GenericImage, DynamicImage};
use matrix4::Matrix4;

pub struct Renderer {
    light_dir: Vector3,
    models: Vec<Model>,
    zbuffer: ZBuffer,
    texture: Box<DynamicImage>,
    timer: f32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { light_dir: Vector3::new(0.0, 0.0, -1.0),
                   models: Vec::new(),
                   zbuffer: ZBuffer::new(800, 800),
                   texture: Box::new(image::open("head_diffuse.png").unwrap()),
                   timer: 0.0 }
    }

    pub fn load_models(&mut self) {
        let model = Model::new("head.obj");
        self.models.push(model);
    }

    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.zbuffer.clear();

        self.timer += 0.1;
        let p = Vector3::new(10.0 * self.timer.cos(), 0.0, 10.0 * self.timer.sin());

        for model in &self.models {
            let mvp = Matrix4::look_at(p, Vector3::new(0.0, 0.0, -1.0), Vector3::new(0.0, 1.0, 0.0)) * Matrix4::translation(0.0, 0.0, -1.0) * Matrix4::scale(0.5, 0.5, 0.5);

            for triangle in model.triangles() {
                let mut triangle_mvp = *triangle;
                triangle_mvp.v0 = mvp * triangle_mvp.v0;
                triangle_mvp.v1 = mvp * triangle_mvp.v1;
                triangle_mvp.v2 = mvp * triangle_mvp.v2;

                let normal = Vector3::cross(triangle.v2 - triangle.v0, triangle.v1 - triangle.v0).normalized();
                let intensity = Vector3::dot(normal, self.light_dir);
                let intensity = if intensity > 0.0 {
                    intensity
                } else {
                    0.0
                };

                Renderer::draw_triangle(canvas, triangle_mvp, &mut self.zbuffer, &self.texture, intensity);
            }
        }
    }

    fn draw_triangle(canvas: &mut Canvas<sdl2::video::Window>, triangle: Triangle, zbuffer: &mut ZBuffer, texture: &Box<DynamicImage>, intensity: f32) {
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
                            
                            if z_distance > zbuffer.sample(x as usize, y as usize) {
                                zbuffer.set(z_distance, x as usize, y as usize);

                                let u = uvw.x * triangle.vt0.x + uvw.y * triangle.vt1.x + uvw.z * triangle.vt2.x;
                                let v = 1.0 - (uvw.x * triangle.vt0.y + uvw.y * triangle.vt1.y + uvw.z * triangle.vt2.y);

                                let w = texture.width() as f32;
                                let h = texture.height() as f32;

                                let color = texture.get_pixel((u * w) as u32, (v * h) as u32);
                                
                                let r = (color.data[0] as f32 * intensity) as u8;
                                let g = (color.data[1] as f32 * intensity) as u8;
                                let b = (color.data[2] as f32 * intensity) as u8;
                                
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
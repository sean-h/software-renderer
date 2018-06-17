extern crate sdl2;

use vector3::Vector3;
use vector2::Vector2i;
use model::Model;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use std::mem;

pub struct Renderer {
    light_dir: Vector3,
    models: Vec<Model>,
    render_target_width: i32,
    render_target_height: i32,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { light_dir: Vector3::new(0.0, 0.0, -1.0),
                   models: Vec::new(),
                   render_target_width: 800,
                   render_target_height: 800 }
    }

    pub fn load_models(&mut self) {
        let model = Model::new("monkey.obj");
        self.models.push(model);
    }

    pub fn render(&self, canvas: &mut Canvas<sdl2::video::Window>) {
        for model in &self.models {
            for t in 0..model.triangle_count() {
                let (v0, v1, v2) = model.get_triangle_vertices(t);
                
                let p0 = Vector2i::new(to_screen_space(v0.x(), self.render_target_width as f32), to_screen_space(-v0.y(), self.render_target_height as f32));
                let p1 = Vector2i::new(to_screen_space(v1.x(), self.render_target_width as f32), to_screen_space(-v1.y(), self.render_target_height as f32));
                let p2 = Vector2i::new(to_screen_space(v2.x(), self.render_target_width as f32), to_screen_space(-v2.y(), self.render_target_height as f32));

                let normal = Vector3::cross(v2.position() - v0.position(), v1.position() - v0.position()).normalized();
                let intensity = Vector3::dot(normal, self.light_dir);

                if intensity > 0.0 {
                    let color_value = (255.0 * intensity) as u8;
                    canvas.set_draw_color(Color::RGB(color_value, color_value, color_value));
                    self.draw_triangle(canvas, p0, p1, p2);
                }
            }
        }
    }

    fn draw_triangle(&self, canvas: &mut Canvas<sdl2::video::Window>, v0: Vector2i, v1: Vector2i, v2: Vector2i) {
        let (bbox_min, bbox_max) = Vector2i::bbox3(v0, v1, v2);

        for x in bbox_min.x..bbox_max.x+1 {
            for y in bbox_min.y..bbox_max.y+1 {
                let b = Vector2i::barycentric(Vector2i::new(x, y), v0, v1, v2);
                match b {
                    Some(uvw) => {
                        if uvw.x >= 0.0 && uvw.y >= 0.0 && uvw.z >= 0.0 {
                            canvas.draw_point(Point::new(x, y)).unwrap();
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
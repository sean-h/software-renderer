extern crate sdl2;

use vector3::Vector3;
use vector2::Vector2i;
use model::{Model, Vertex};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use std::mem;
use zbuffer::ZBuffer;

pub struct Renderer {
    light_dir: Vector3,
    models: Vec<Model>,
    render_target_width: i32,
    render_target_height: i32,
    zbuffer: ZBuffer,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { light_dir: Vector3::new(0.0, 0.0, -1.0),
                   models: Vec::new(),
                   render_target_width: 800,
                   render_target_height: 800,
                   zbuffer: ZBuffer::new(800, 800) }
    }

    pub fn load_models(&mut self) {
        let model = Model::new("head.obj");
        self.models.push(model);
    }

    pub fn render(&mut self, canvas: &mut Canvas<sdl2::video::Window>) {
        self.zbuffer.clear();

        for model in &self.models {
            for t in 0..model.triangle_count() {
                let (v0, v1, v2) = model.get_triangle_vertices(t);
                
                let normal = Vector3::cross(v2.position() - v0.position(), v1.position() - v0.position()).normalized();
                let intensity = Vector3::dot(normal, self.light_dir);

                if intensity > 0.0 {
                    let color_value = (255.0 * intensity) as u8;
                    canvas.set_draw_color(Color::RGB(color_value, color_value, color_value));
                    Renderer::draw_triangle(canvas, v0, v1, v2, &mut self.zbuffer);
                }
            }
        }
    }

    fn draw_triangle(canvas: &mut Canvas<sdl2::video::Window>, v0: Vertex, v1: Vertex, v2: Vertex, zbuffer: &mut ZBuffer) {
        let canvas_width = canvas.viewport().width() as f32;
        let canvas_height = canvas.viewport().height() as f32;

        let p0 = Vector2i::new(to_screen_space(v0.x, canvas_width as f32), to_screen_space(-v0.y, canvas_height as f32));
        let p1 = Vector2i::new(to_screen_space(v1.x, canvas_width as f32), to_screen_space(-v1.y, canvas_height as f32));
        let p2 = Vector2i::new(to_screen_space(v2.x, canvas_width as f32), to_screen_space(-v2.y, canvas_height as f32));
        
        let ortho0 = Vector3::new((v0.x + 1.0) * canvas_width / 2.0, (-v0.y + 1.0) * canvas_height / 2.0, 0.0);
        let ortho1 = Vector3::new((v1.x + 1.0) * canvas_width / 2.0, (-v1.y + 1.0) * canvas_height / 2.0, 0.0);
        let ortho2 = Vector3::new((v2.x + 1.0) * canvas_width / 2.0, (-v2.y + 1.0) * canvas_height / 2.0, 0.0);

        let (bbox_min, bbox_max) = Vector2i::bbox3(p0, p1, p2);

        for x in bbox_min.x..bbox_max.x+1 {
            for y in bbox_min.y..bbox_max.y+1 {
                if x >= canvas_width as i32 || y >= canvas_height as i32 {
                    continue;
                }

                let b = Vector3::barycentric(Vector3::new(x as f32, y as f32, 0.0), ortho0, ortho1, ortho2);

                match b {
                    Some(uvw) => {
                        if uvw.x >= 0.0 && uvw.y >= 0.0 && uvw.z >= 0.0 {
                            let z_distance = uvw.x * v0.z + uvw.y * v1.z + uvw.z * v2.z;
                            if z_distance < zbuffer.sample(x as usize, y as usize) {
                                zbuffer.set(z_distance, x as usize, y as usize);
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
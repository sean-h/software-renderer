extern crate sdl2;
extern crate image;
extern crate toml;
extern crate cmdpro;
extern crate tdmath;
extern crate modelloader;

pub mod model;
pub mod renderer;
pub mod zbuffer;
pub mod camera;
pub mod settings;
pub mod material;
pub mod text;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::{Duration, SystemTime};
pub use renderer::Renderer;
use settings::Settings;
use cmdpro::{CommandLineProcessor, ParameterType, ParameterValue};
use text::*;
use std::collections::HashMap;
use tdmath::Vector2i;

fn main() {
    let mut command_line_processor = CommandLineProcessor::new();
    command_line_processor.add_parameter("model", ParameterType::Path, vec!["--model".to_owned(), "--m".to_owned()]);
    command_line_processor.add_parameter("material", ParameterType::Path, vec!["--material".to_owned()]);
    command_line_processor.add_parameter("width", ParameterType::UInteger, vec!["--width".to_owned(), "--w".to_owned()]);
    command_line_processor.add_parameter("height", ParameterType::UInteger, vec!["--height".to_owned(), "--h".to_owned()]);
    command_line_processor.set_help_text(include_str!("help.txt"));
    command_line_processor.parse_command_line();

    if command_line_processor.abort_flag() {
        return;
    }

    let settings = Settings::from_commandline(&command_line_processor);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem.window("Software Renderer", settings.width(), settings.height())
        .position_centered()
        .resizable()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();    

    let mut renderer = Renderer::new();
    renderer.load_models(vec!(settings.model_path()));

    let font = ttf_context.load_font("fonts/UbuntuMono-R.ttf", 16).unwrap();
    let mut text_map = HashMap::new();

    let mut proj_text = Text::new();
    proj_text.set_text(&font, &texture_creator, &format!("(P)rojection: {}", renderer.projection_mode_str()), Color::RGBA(255, 0, 0, 255));
    proj_text.set_offset(Vector2i::new(0, -50));
    proj_text.set_anchor(Anchor::BottomLeft);
    text_map.insert(TextID::Projection, proj_text);

    let mut smooth_text = Text::new();
    smooth_text.set_text(&font, &texture_creator, &format!("(S)mooth Shading: {}", renderer.smooth_shading_str()), Color::RGBA(255, 0, 0, 255));
    smooth_text.set_offset(Vector2i::new(0, -25));
    smooth_text.set_anchor(Anchor::BottomLeft);
    text_map.insert(TextID::SmoothShading, smooth_text);

    match command_line_processor.get_parameter_value("material") {
        ParameterValue::Path(material_path) => renderer.load_material(material_path),
        _ => ()
    }

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut mouse_down = false;
    // Set camera position
    renderer.orbit(0.0, 0.0);
    let target_frame_rate = 1_000_000_000u32 / 60;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let frame_start_time = SystemTime::now();

        canvas.set_draw_color(Color::RGB(65, 65, 65));
        canvas.clear();

        // Event Handler
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(x, y) => {
                            renderer.resize(x as usize, y as usize);
                        },
                        _ => ()
                    }
                },
                Event::KeyDown { keycode: key, .. } => {
                    match key {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::P) => {
                            renderer.toggle_projection_mode();
                            if let Some(text) = text_map.get_mut(&TextID::Projection) {
                                text.set_text(&font, &texture_creator, &format!("(P)rojection: {}", renderer.projection_mode_str()), Color::RGBA(255, 0, 0, 255))
                            }
                        },
                        Some(Keycode::Equals) => renderer.increase_ambient_intensity(0.1),
                        Some(Keycode::Minus) => renderer.increase_ambient_intensity(-0.1),
                        Some(Keycode::S) => {
                            renderer.toggle_smooth_shading();
                            if let Some(text) = text_map.get_mut(&TextID::SmoothShading) {
                                text.set_text(&font, &texture_creator, &format!("(S)mooth Shading: {}", renderer.smooth_shading_str()), Color::RGBA(255, 0, 0, 255));
                            }
                        },
                        _ => (),
                    }
                },
                Event::MouseWheel { y: mouse_y, .. } => {
                    let zoom_sensitivity = 4.0;
                    renderer.zoom_camera(-mouse_y as f32 * zoom_sensitivity);
                },
                Event::MouseButtonDown { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        mouse_down = true;
                    }
                },
                Event::MouseButtonUp { mouse_btn, .. } => {
                    if mouse_btn == MouseButton::Left {
                        mouse_down = false;
                    }
                },
                Event::MouseMotion { xrel: delta_x, .. } => {
                    if mouse_down {
                        let sensitivity = 0.01;
                        renderer.orbit(delta_x as f32 * sensitivity, 0.0);
                    }
                }
                _ => {}
            }
        }

        renderer.render(&mut canvas);

        for (_, text) in text_map.iter() {
            match text.texture() {
                Some(texture) => {
                    let position = text.get_position(canvas.viewport().width(), canvas.viewport().height());
                    canvas.copy(&texture, None, Some(sdl2::rect::Rect::new(position.x, position.y, text.width().unwrap(), text.height().unwrap()))).unwrap();
                },
                None => ()
            }
        }
        
        canvas.present();

        match frame_start_time.elapsed() {
            Ok(t) => {
                if t.as_secs() < 1 && t.subsec_nanos() < target_frame_rate {
                    ::std::thread::sleep(Duration::new(0, target_frame_rate - t.subsec_nanos()));
                }
            },
            Err(e) => println!("Unable to determine render time: {}", e),
        }
    }
}





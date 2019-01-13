extern crate sdl2;
extern crate image;
extern crate toml;
extern crate cmdpro;
extern crate tdmath;
extern crate modelloader;

mod model;
mod renderer;
mod zbuffer;
mod camera;
mod settings;
mod material;

use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::Duration;
pub use renderer::Renderer;
use settings::Settings;
use cmdpro::{CommandLineProcessor, ParameterType, ParameterValue};

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
 
    let window = video_subsystem.window("Software Renderer", settings.width(), settings.height())
        .position_centered()
        .resizable()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut renderer = Renderer::new();
    renderer.load_models(vec!(settings.model_path()));

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

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
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
                            //canvas.window_mut().set_size(x as u32, y as u32).unwrap();
                            renderer.resize(x as usize, y as usize);
                        },
                        _ => ()
                    }
                },
                Event::KeyDown { keycode: key, .. } => {
                    match key {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::P) => renderer.toggle_projection_mode(),
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
                        renderer.orbit(-delta_x as f32 * sensitivity, 0.0);
                    }
                }
                _ => {}
            }
        }

        renderer.render(&mut canvas);
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}





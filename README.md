# Software Renderer

A software renderer written in Rust able to display .obj models in real-time without using OpenGL/DirectX. It is based on [tinyrenderer](https://github.com/ssloy/tinyrenderer).

## Dependencies
- [sdl2](https://github.com/Rust-SDL2/rust-sdl2)
- [image](https://github.com/PistonDevelopers/image)
- [toml](https://github.com/alexcrichton/toml-rs)
- [tdmath](https://github.com/sean-h/tdmath)
- [cmdpro](https://github.com/sean-h/cmdpro)
- [modelloader](https://github.com/sean-h/modelloader)

## Usage

Run the following to display a test model. The `--model` parameter specifies the model path and `--material` specifies the material path.

```
cargo run --release -- --model models/monkey.obj --material models/color_grid.toml
```

## Examples

![Head](https://github.com/sean-h/software-renderer/blob/master/screenshots/head.png)
![Monkey](https://github.com/sean-h/software-renderer/blob/master/screenshots/monkey.png)

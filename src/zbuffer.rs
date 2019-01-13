use std::f32;

pub struct ZBuffer {
    buffer: Vec<f32>,
    width: usize,
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> ZBuffer {
        ZBuffer { buffer: vec![0.0; (width * height) as usize], width: width }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.buffer = vec![0.0; (width * height) as usize];
    }

    pub fn sample(&self, x: usize, y: usize) -> f32 {
        self.buffer[x + self.width * y]
    }

    pub fn set(&mut self, value: f32, x: usize, y: usize) {
        self.buffer[x + self.width * y] = value;
    }

    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = f32::MAX;
        }
    }
}
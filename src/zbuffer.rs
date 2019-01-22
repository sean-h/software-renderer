//! ZBuffer

use std::f32;

/// ZBuffer
pub struct ZBuffer {
    buffer: Vec<f32>,
    width: usize,
}

impl ZBuffer {
    /// Returns a new `ZBuffer` with `width` and `height`.
    pub fn new(width: usize, height: usize) -> ZBuffer {
        ZBuffer { buffer: vec![0.0; (width * height) as usize], width: width }
    }

    /// Resizes this `ZBuffer`.
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.buffer = vec![0.0; (width * height) as usize];
    }

    /// Returns the value at `x`, `y`.
    pub fn sample(&self, x: usize, y: usize) -> f32 {
        self.buffer[x + self.width * y]
    }

    /// Sets the value at `x`, `y`.
    pub fn set(&mut self, value: f32, x: usize, y: usize) {
        self.buffer[x + self.width * y] = value;
    }

    /// Sets all values to `f32::MAX`.
    pub fn clear(&mut self) {
        for i in 0..self.buffer.len() {
            self.buffer[i] = f32::MAX;
        }
    }
}
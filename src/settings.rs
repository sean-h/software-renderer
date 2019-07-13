//! Application Settings

extern crate cmdpro;

use std::path::{Path, PathBuf};
use cmdpro::{CommandLineProcessor, ParameterValue};

/// Application Settings
pub struct Settings {
    model_path: PathBuf,
    width: u32,
    height: u32,
}

impl Settings {
    /// Returns a new `Settings` from a `CommandLineProcessor`.
    /// Default values are used if not passed into the program.
    pub fn from_commandline(commandline: &CommandLineProcessor) -> Settings {
        let model_path = match commandline.get_parameter_value("model") {
            ParameterValue::Path(path) => path,
            _ => panic!("Model path not set"),
        };

        let width = match commandline.get_parameter_value("width") {
            ParameterValue::UInteger(width) => *width,
            _ => 800,
        };

        let height = match commandline.get_parameter_value("height") {
            ParameterValue::UInteger(height) => *height,
            _ => 800,
        };

        Settings {
            model_path: PathBuf::from(model_path),
            width,
            height,
        }
    }

    /// Returns the model path.
    pub fn model_path(&self) -> &Path {
        self.model_path.as_path()
    }

    /// Returns the target window width.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the target window height.
    pub fn height(&self) -> u32 {
        self.height
    }
}

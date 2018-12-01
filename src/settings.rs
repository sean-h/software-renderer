extern crate cmdpro;

use std::path::{Path, PathBuf};
use cmdpro::{CommandLineProcessor, ParameterValue};

pub struct Settings {
    model_path: PathBuf,
    width: u32,
    height: u32,
}

impl Settings {
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
            width: width,
            height: height,
        }
    }

    pub fn model_path(&self) -> &Path {
        self.model_path.as_path()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

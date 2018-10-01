use std::path::PathBuf;
use std::collections::HashMap;
use std::env;

pub enum ParameterType {
    Flag,
    UInteger,
    Path,
}

pub enum ParameterValue {
    None,
    Flag,
    UInteger(u32),
    Path(PathBuf),
}

struct Parameter {
    pub parameter_name: String,
    pub parameter_type: ParameterType,
    pub aliases: Vec<String>,
    value: ParameterValue,
}

pub struct CommandLineProcessor {
    parameters: HashMap<String, Parameter>,
    help_text: Option<String>,
    version_text: Option<String>,
    abort_flag: bool,
}

impl CommandLineProcessor {
    pub fn new() -> CommandLineProcessor {
        CommandLineProcessor {
            parameters: HashMap::new(),
            help_text: None,
            version_text: None,
            abort_flag: false,
        }
    }

    pub fn add_parameter(&mut self, parameter_name: &str, parameter_type: ParameterType, aliases: Vec<String>) {
        let parameter = Parameter {
            parameter_name: parameter_name.to_owned(),
            parameter_type,
            aliases,
            value: ParameterValue::None,
        };

        self.parameters.insert(parameter_name.to_owned(), parameter);
    }

    pub fn parse_command_line(&mut self) {
        let mut iter = env::args();
        iter.next(); // Skip executable name

        loop {
            match iter.next() {
                Some(argument) => {
                    match argument.as_ref() {
                        "--help" => {
                            self.print_help_text();
                            self.abort_flag = true;
                        },
                        "--h" => {
                            self.print_help_text();
                            self.abort_flag = true;
                        },
                        "--version" => {
                            self.print_version_text();
                            self.abort_flag = true;
                        },
                        "--v" => {
                            self.print_version_text();
                            self.abort_flag = true;
                        },
                        arg => {
                            let mut parameter_exists = false;

                            for (name, parameter) in self.parameters.iter_mut() {
                                if parameter.aliases.iter().any(|x| x == arg) {
                                    parameter_exists = true;

                                    match parameter.parameter_type {
                                        ParameterType::Flag => parameter.value = ParameterValue::Flag,
                                        ParameterType::UInteger => {
                                            match iter.next() {
                                                Some(val) => {
                                                    match val.parse::<u32>() {
                                                        Ok(val) => parameter.value = ParameterValue::UInteger(val),
                                                        Err(err) => panic!(format!("Unable to convert parameter {} to unsigned integer\n{}", name, err))
                                                    }
                                                    
                                                },
                                                None => panic!(format!("No value passed for parameter {}", name)),
                                            }
                                        },
                                        ParameterType::Path => {
                                            match iter.next() {
                                                Some(val) => {
                                                    let mut path = PathBuf::new();
                                                    path.push(val);
                                                    parameter.value = ParameterValue::Path(path);
                                                },
                                                None => panic!(format!("No value passed for parameter {}", name)),
                                            }
                                        },
                                    }
                                }
                            }

                            if !parameter_exists {
                                println!("Unknown parameter: {}", arg);
                                self.abort_flag = true;
                            }
                        },
                    }
                },
                None => break,
            }
        }
    }

    pub fn set_help_text(&mut self, help_text: &str) {
        self.help_text = Some(help_text.to_owned());
    }

    fn print_help_text(&self) {
        match &self.help_text {
            Some(help_text) => println!("{}", help_text),
            None => println!("No help text has been set."),
        }
    }

    pub fn set_version_text(&mut self, version_text: &str) {
        self.version_text = Some(version_text.to_owned());
    }

    fn print_version_text(&self) {
        match &self.version_text {
            Some(version_text) => println!("{}", version_text),
            None => println!("No version text has been set."),
        }
    }

    pub fn get_parameter_value(&self, parameter_name: &str) -> &ParameterValue {
        match self.parameters.get(parameter_name) {
            Some(parameter) => &parameter.value,
            None => &ParameterValue::None,
        }
    }

    pub fn abort_flag(&self) -> bool {
        self.abort_flag
    }
}
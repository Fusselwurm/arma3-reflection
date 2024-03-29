use core::option::Option::Some;
use std::collections::HashMap;
use std::io::Error;
use std::fs::File;
use std::io::Read;
use std::string::String;
use crate::args_parser::ArgsParser;

fn read_string(mut r: File) -> Result<String, Error> {
    let mut contents: String = String::new();
    let bytes_read = r.read_to_string(&mut contents);
    match bytes_read {
        Ok(count) => {println!("read {} bytes from params file", count); Ok(contents)},
        Err(e) => {println!("could not read file: {}", e); Err(e)},
    }
}

pub fn get_params_file_contents(filename: &str) -> Vec<String> {
    let file_read_result = File::open(filename).and_then(read_string);

    file_read_result.unwrap_or("".to_string())
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub struct StartupParameters {
    env_args_parser: ArgsParser,
    parameter_file_args_parser: ArgsParser,
}

impl StartupParameters {
    pub fn new(env_args_parser: ArgsParser) -> StartupParameters {
        // TODO check if first or last -par value is taken
        let parfile_args: Vec<String> = match env_args_parser.options().get("par").unwrap_or(&Vec::new()).last() {
            Some(filename) => {
                get_params_file_contents(filename)
            }
            None => Vec::new()
        };

        let parameter_file_args_parser: ArgsParser = ArgsParser::new(parfile_args);
        StartupParameters { env_args_parser, parameter_file_args_parser }
    }

    pub fn options(&self) -> HashMap<String, Vec<String>> {
        let mut env_opts: HashMap<String, Vec<String>> = self.env_args_parser.options();
        let par_opts: HashMap<String, Vec<String>> = self.parameter_file_args_parser.options();

        env_opts.extend(par_opts);
        env_opts
    }

    pub fn arguments(&self) -> Vec<String> {
        let mut env_args = self.env_args_parser.arguments();
        let par_args = self.parameter_file_args_parser.arguments();

        env_args.extend(par_args);
        env_args
    }
}

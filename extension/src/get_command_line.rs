use crate::args_parser::ArgsParser;
use std::collections::HashMap;
use crate::startup_parameters::StartupParameters;

fn get_args_parser(args: &Vec<String>) -> ArgsParser {
    ArgsParser::new(args)
}

pub fn get_command_line_opts(args: &Vec<String>) -> HashMap<String, Vec<String>>{
    StartupParameters::new(get_args_parser(args)).get_options()
}

pub fn get_command_line_args(args: &Vec<String>) -> Vec<String> {
    StartupParameters::new(get_args_parser(args)).get_arguments()
}

pub fn get_argument(args: &Vec<String>, index: usize) -> String {
    let foo = get_command_line_args(args);
    match foo.get(index) {
        Some(f) => f.to_string(),
        None => "".to_string(),
    }
}

pub fn get_option_first(args: &Vec<String>, name: &String) -> String {
    let foo = get_command_line_opts(args);
    match foo.get(name) {
        Some(f) => f.first().unwrap_or(&"".to_string()).to_string(),
        None => "".to_string(),
    }
}

pub fn get_option(args: &Vec<String>, name: &String) -> Vec<String> {
    get_command_line_opts(args).get(name).unwrap_or(&Vec::new()).to_vec()
}

pub fn get_command_line_raw(args: &Vec<String>) -> String {
    args.join(" ")
}

use crate::args_parser::ArgsParser;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use crate::startup_parameters::StartupParameters;

fn get_args_parser() -> ArgsParser {
    ArgsParser::new(&env::args().collect())
}

pub fn get_command_line_opts() -> HashMap<String, String>{
    StartupParameters::new(get_args_parser()).get_options()
}

pub fn get_command_line_args() -> Vec<String> {
    StartupParameters::new(get_args_parser()).get_arguments()
}

pub fn get_command_line_raw() -> String {
    get_command_line_args().join(" ")
}

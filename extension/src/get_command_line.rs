use crate::args_parser::ArgsParser;
use std::collections::HashMap;
use std::env;
use crate::startup_parameters::StartupParameters;

fn get_args_parser(args: &Vec<String>) -> ArgsParser {
    ArgsParser::new(args)
}

pub fn get_command_line_opts(args: &Vec<String>) -> HashMap<String, String>{
    StartupParameters::new(get_args_parser(args)).get_options()
}

pub fn get_command_line_args(args: &Vec<String>) -> Vec<String> {
    StartupParameters::new(get_args_parser(args)).get_arguments()
}

pub fn get_command_line_raw(args: &Vec<String>) -> String {
    args.join(" ")
}

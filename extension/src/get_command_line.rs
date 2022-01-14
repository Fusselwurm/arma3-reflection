use std::cmp::{min};
use crate::args_parser::ArgsParser;
use std::collections::HashMap;
use crate::startup_parameters::StartupParameters;

fn get_args_parser(args: &Vec<String>) -> ArgsParser {
    ArgsParser::new(args)
}

pub struct Commandline {
    args: Vec<String>,
    max_return_str_len: usize,
}

impl Commandline {

    pub fn new(args: Vec<String>, max_return_str_len: usize) -> Commandline {
        Commandline { args, max_return_str_len }
    }

    pub fn get_command_line_opts(&self) -> HashMap<String, Vec<String>>{
        StartupParameters::new(get_args_parser(&self.args)).get_options()
    }

    pub fn get_command_line_args(&self) -> Vec<String> {
        StartupParameters::new(get_args_parser(&self.args)).get_arguments()
    }

    pub fn get_argument(&self, index: usize) -> String {
        let foo = self.get_command_line_args();
        match foo.get(index) {
            Some(f) => f.to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_command_line_raw_idx(&self, index: usize) -> String {
        let s = self.get_command_line_raw();
        let from = min(s.len(), index * self.max_return_str_len);
        let to = min(s.len(), (index + 1) * self.max_return_str_len);
        s[from..to].to_string()
    }

    pub fn get_option_first(&self, name: &String) -> String {
        let foo = self.get_command_line_opts();
        match foo.get(name) {
            Some(f) => f.first().unwrap_or(&"".to_string()).to_string(),
            None => "".to_string(),
        }
    }

    pub fn get_option(&self, name: &String) -> Vec<String> {
        self.get_command_line_opts().get(name).unwrap_or(&Vec::new()).to_vec()
    }

    pub fn get_command_line_raw(&self) -> String {
        self.args.join(" ")
    }

}


mod args_parser;
mod get_command_line;
mod startup_parameters;
mod lib_test;
mod startup_parameters_test;
mod get_command_line_test;
mod args_parser_test;

use std::env;
use arma_rs::{rv, rv_handler};

fn args() -> Vec<String> {
    env::args().collect()
}

#[rv]
#[allow(dead_code)]
fn get_option_first(name: &String) -> String {
    let foo = get_command_line::get_command_line_opts(&args());
    match foo.get(name) {
        Some(f) => f.to_string(),
        None => "".to_string(),
    }
}

#[rv]
#[allow(dead_code)]
fn get_argument(index: usize) -> String {
    let foo = get_command_line::get_command_line_args(&args());
    match foo.get(index) {
        Some(f) => f.to_string(),
        None => "".to_string(),
    }
}

/// just an alias, and a deprecated one
#[rv]
#[allow(dead_code)]
#[deprecated(note = "do use get_option_first(name) instead")]
fn arg(name: &String) -> String {
    get_option_first(name)
}

#[rv]
#[allow(dead_code)]
fn get_cmdline_raw() -> String {
    get_command_line::get_command_line_raw(&args())
}


#[rv_handler]
fn init() {}

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
    get_command_line::get_option_first(&args(), name)
}

#[rv]
#[allow(dead_code)]
fn get_argument(index: usize) -> String {
    get_command_line::get_argument(&args(), index)
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

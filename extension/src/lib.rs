mod args_parser;
mod command_line;
mod startup_parameters;
mod lib_test;
mod startup_parameters_test;
mod command_line_test;
mod args_parser_test;

use std::env;
use arma_rs::{Extension, arma};
use crate::command_line::Commandline;

fn get_command_line() -> Commandline {
    Commandline::new(env::args().collect(), (10240 - 8) / 4)
}

fn get_option_first(name: String) -> String {
    get_command_line().get_option_first(&name)
}

fn get_option(name: String) -> Vec<String> {
    get_command_line().get_option(&name)
}

fn get_argument(index: u16) -> String {
    get_command_line().get_argument(index as usize)
}


fn get_commandline_raw(index: u16) -> String {
    get_command_line().get_command_line(index as usize)
}

fn get_string_of_len(len: u32) -> String {
    "x".to_string().repeat(len as usize)
}

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("get_commandline_raw", get_commandline_raw)
        .command("get_argument", get_argument)
        .command("get_option_first", get_option_first)
        .command("get_option", get_option)
        .command("get_string_of_len", get_string_of_len)
        .finish()
}

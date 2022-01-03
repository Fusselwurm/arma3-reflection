mod args_parser;
mod get_command_line;
mod startup_parameters;
mod lib_test;
mod startup_parameters_test;
mod get_command_line_test;
mod args_parser_test;

use std::env;
use arma_rs::{Extension, arma};

fn args() -> Vec<String> {
    env::args().collect()
}

fn get_option_first(name: String) -> String {
    get_command_line::get_option_first(&args(), &name)
}

fn get_argument(index: String) -> String {

    get_command_line::get_argument(&args(), index.parse().unwrap())
}

fn get_cmdline_raw() -> String {
    get_command_line::get_command_line_raw(&args())
}

#[arma]
fn init() -> Extension {
    Extension::build()
        .command("get_cmdline_raw", get_cmdline_raw)
        .command("get_argument", get_argument)
        .command("get_option_first", get_option_first)
        .finish()
}

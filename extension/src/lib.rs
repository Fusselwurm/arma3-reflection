mod args_parser;
mod command_line;
mod startup_parameters;
mod lib_test;
mod startup_parameters_test;
mod command_line_test;
mod args_parser_test;

use std::env;
use arma_rs::{Extension, arma, Group};
use crate::command_line::Commandline;

fn get_command_line() -> Commandline {
    Commandline::new(env::args().collect(), (10240 - 8) / 4)
}

fn option(name: String) -> Vec<String> {
    get_command_line().get_option(&name)
}

fn argument(index: u16) -> String {
    get_command_line().get_argument(index as usize)
}


fn raw(index: u16) -> String {
    get_command_line().get_command_line(index as usize)
}

#[arma]
fn init() -> Extension {
    Extension::build()
        .group("commandline", Group::new()
            .command("raw", raw)
            .command("argument", argument)
            .command("option", option)
        )
        .finish()
}

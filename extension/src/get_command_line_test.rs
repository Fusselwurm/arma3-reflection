use crate::args_parser::ArgsParser;
use std::collections::HashMap;
use std::env;
use crate::startup_parameters::StartupParameters;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod get_command_line_test {
    use std::collections::HashMap;
    use std::env;
    use crate::get_command_line::{get_command_line_args, get_command_line_opts, get_command_line_raw};

    #[test]
    fn raw_returns_complete_command_line() {
        assert_eq!(get_command_line_raw(&vec_of_strings!["foo", "bar", "-baz=x;"]), "foo bar -baz=x;")
    }


    #[test]
    fn get_command_line_opts_gets_all_opts() {
        assert_eq!(
            get_command_line_opts(&vec_of_strings!["foo", "-bar", "-baz=x"]),
            HashMap::from([("bar".to_string(), "".to_string()), ("baz".to_string(), "x".to_string())])
        )
    }

    #[test]
    fn get_command_line_args_gets_all_non_opt_args() {
        assert_eq!(
            get_command_line_args(&vec_of_strings!["foo", "-bar", "-baz=x", "bork"]),
            vec_of_strings!["foo", "bork"]
        )
    }
}

mod args_parser;
mod get_command_line;

use arma_rs::{rv, rv_handler};

#[cfg(test)]
mod tests {
    use crate::{get_argument, get_option_first, init};
    use crate::get_command_line::{get_command_line};

    #[test]
    fn init_exists() {
        init();
    }

    #[test]
    fn get_parameter_does_not_fail() {
        assert!(get_option_first(&("foo").to_string()).eq(""));
    }

    #[test]
    fn get_argument_does_not_fail() {
        assert!(get_argument(999).eq(""));
    }

    #[test]
    fn get_argument_gets_arg() {
        assert!(
            get_argument(0).eq(get_command_line().arguments.get(0).expect("wtf")),
            "0 = {:?}",
            get_command_line().arguments.get(0).ok_or("EMPTY")
        )
    }

    #[test]
    fn get_parameter_gets_par() {
        assert!(
            get_option_first(&("-format").to_string()).eq("json") || get_option_first(&("-format").to_string()).eq("xml"),
            "--format={}",
            get_option_first(&("-format").to_string()).to_string()
        );
    }
}

#[rv]
#[allow(dead_code)]
fn get_option_first(name: &String) -> String {
    let foo = get_command_line::get_command_line();
    match foo.options.get(name) {
        Some(f) => f.to_string(),
        None => "".to_string(),
    }
}

#[rv]
#[allow(dead_code)]
fn get_argument(index: usize) -> String {
    let foo = get_command_line::get_command_line();
    match foo.arguments.get(index) {
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
    get_command_line::get_command_line_raw()
}


#[rv_handler]
fn init() {}

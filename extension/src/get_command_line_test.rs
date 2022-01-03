macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod get_command_line_test {
    use std::collections::HashMap;
    use crate::get_command_line::{get_argument, get_command_line_args, get_command_line_opts, get_command_line_raw, get_option_first};

    #[test]
    fn get_command_line_raw_returns_complete_command_line() {
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

    #[test]
    fn get_option_first_gets_option() {
        assert_eq!(
            get_option_first(&vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz=z"], &"bar".to_string()),
            "x".to_string()
        );
        assert_eq!(
            get_option_first(&vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz=z"], &"zoom".to_string()),
            "".to_string()
        );
    }

    #[test]
    fn get_argument_gets_arg_of_index() {
        assert_eq!(
            get_argument(&vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz=z"], 0),
            "foo".to_string()
        );

        assert_eq!(
            get_argument(&vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz=z"], 15),
            "".to_string()
        );
    }

}

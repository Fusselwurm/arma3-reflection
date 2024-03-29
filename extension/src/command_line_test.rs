macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod get_command_line_test {
    use std::collections::HashMap;
    use crate::Commandline;

    #[test]
    fn get_command_line_raw_returns_complete_command_line() {
        let commandline: Commandline = Commandline::new(vec_of_strings!["foo", "bar", "-baz=x;"], 50);
        assert_eq!(commandline.get_command_line(0), "foo bar -baz=x;")
    }


    #[test]
    fn get_command_line_opts_gets_all_opts() {
        let commandline: Commandline = Commandline::new(vec_of_strings!["foo", "-bar", "-baz=x"], 1024);
        assert_eq!(
            commandline.get_command_line_opts(),
            HashMap::from([("bar".to_string(), vec!["".to_string()]), ("baz".to_string(), vec!["x".to_string()])])
        )
    }

    #[test]
    fn get_command_line_args_gets_all_args() {
        let commandline: Commandline = Commandline::new(vec_of_strings!["foo", "-bar", "-baz=x", "bork"], 1024);
        assert_eq!(
            commandline.get_command_line_args(),
            vec_of_strings!["foo", "-bar", "-baz=x", "bork"]
        )
    }

    #[test]
    fn get_option_gets_option_vec() {
        let pars = vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz", "-baz=a"];
        let commandline: Commandline = Commandline::new(pars.clone(), 1024);
        assert_eq!(
            commandline.get_option(&"bar".to_string()),
            vec_of_strings!["x", "y"]
        );
        assert_eq!(
            commandline.get_option(&"baz".to_string()),
            vec_of_strings!["", "a"]
        );
    }

    #[test]
    fn get_argument_gets_arg_of_index() {
        let commandline: Commandline = Commandline::new(vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz=z"], 1024);
        assert_eq!(
            commandline.get_argument(0),
            "foo".to_string()
        );

        assert_eq!(
            commandline.get_argument(15),
            "".to_string()
        );
    }

    #[test]
    fn get_command_line_raw_idx() {
        let commandline: Commandline = Commandline::new(
            vec_of_strings!["foo", "-bar=x", "-bar=y", "-baz=z"],
            5
        );

        [
            (0, "foo -"),
            (1, "bar=x"),
            (2, " -bar"),
            (3, "=y -b"),
            (4, "az=z"),
            (5, ""),
            (6, ""),
        ].iter().for_each(|e| {
            assert_eq!(
                commandline.get_command_line(e.0),
                e.1.to_string()
            );
        });
    }
}

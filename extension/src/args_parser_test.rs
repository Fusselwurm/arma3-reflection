macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod args_parser_test {
    use crate::args_parser::ArgsParser;

    #[test]
    fn options_recognizes_parameter_values() {
        let parser = ArgsParser::new(vec_of_strings!["/something/bin/arma3", "-port=2302", "-mod=foo;"]);

        let map = parser.options();
        let port = map.get("port").expect("no port value in result");
        assert_eq!(1, port.len());
        assert_eq!(port.get(0).unwrap().to_string(), "2302".to_string());
    }

    #[test]
    fn options_returns_empty_string_as_value_for_flags() {
        let parser = ArgsParser::new(vec_of_strings!["/something/bin/arma3", "-foo", "bar"]);
        let map = parser.options();
        let foo = map.get("foo").expect("no foo key o.O");

        assert_eq!(1, foo.len());
        assert_eq!(foo.get(0).unwrap().to_string(), "".to_string());
    }

    #[test]
    fn options_omits_arguments() {
        let parser = ArgsParser::new(vec_of_strings!["/something/bin/arma3", "foo"]);

        let map = parser.options();

        assert!(!map.contains_key("foo"));
    }

    #[test]
    fn options_also_splits_at_newlines() {
        let parser = ArgsParser::new(vec_of_strings!["-a=1\n-b=2"]);
        assert_eq!(
            parser.options().get("a"),
            Some(&vec!["1".to_string()])
        );
        assert_eq!(
            parser.options().get("b"),
            Some(&vec!["2".to_string()])
        );
        assert_eq!(
            parser.options().len(),
            2
        );
    }

    #[test]
    fn get_arguments_gets_arguments() {
        let parser = ArgsParser::new(vec_of_strings!["/something/bin/arma3", "foo", "bar", "-baz=boom"]);

        assert_eq!(
            parser.arguments(),
            vec_of_strings!["/something/bin/arma3", "foo", "bar", "-baz=boom"]
        );
    }

    #[test]
    fn arguments_does_split_at_newline() {
        let parser = ArgsParser::new(vec_of_strings!["/something/bin/arma3", "-foo\n-bar\r-baz", "-baz=boom"]);

        assert_eq!(
            parser.arguments(),
            vec_of_strings!["/something/bin/arma3", "-foo", "-bar", "-baz", "-baz=boom"]
        );
    }

    #[test]
    fn arguments_does_count_consecutive_newlines_as_one_delimiter()
    {
        let parser = ArgsParser::new(vec_of_strings!["foo\n\r\nbar", "\nbaz"]);
        assert_eq!(
            parser.arguments(),
            vec_of_strings!("foo", "bar", "baz")
        )
    }
}

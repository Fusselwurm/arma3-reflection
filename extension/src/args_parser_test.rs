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
    fn get_arguments_gets_arguments() {
        let parser = ArgsParser::new(vec_of_strings!["/something/bin/arma3", "foo", "bar", "-baz=boom"]);

        assert_eq!(
            parser.arguments(),
            vec_of_strings!["/something/bin/arma3", "foo", "bar", "-baz=boom"]
        );
    }
}

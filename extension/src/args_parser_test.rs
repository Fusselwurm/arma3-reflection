#[cfg(test)]
mod args_parser_test {
    use crate::args_parser::ArgsParser;

    #[test]
    fn get_parameters_recognizes_parameter_values() {
        let args: Vec<&str> = vec!["/something/bin/arma3", "-port=2302", "-mod=foo;"];

        let v = args.into_iter().map(|s| s.to_string()).collect();

        let parser = ArgsParser::new(&v);

        let map = parser.get_options();
        let port = map.get("port").expect("no port value in result");
        assert_eq!(1, port.len());
        assert_eq!(port.get(0).unwrap().to_string(), "2302".to_string());
    }

    #[test]
    fn get_parameters_returns_empty_string_as_value_for_flags() {
        let args: Vec<&str> = vec!["/something/bin/arma3", "-foo", "bar"];
        let parser = ArgsParser::new(&args.into_iter().map(|s| s.to_string()).collect());
        let map = parser.get_options();
        let foo = map.get("foo").expect("no foo key o.O");

        assert_eq!(1, foo.len());
        assert_eq!(foo.get(0).unwrap().to_string(), "".to_string());
    }

    #[test]
    fn get_parameters_omits_arguments() {
        let args: Vec<&str> = vec!["/something/bin/arma3", "foo"];
        let parser = ArgsParser::new(&args.into_iter().map(|s| s.to_string()).collect());

        let map = parser.get_options();

        assert!(!map.contains_key("foo"));
    }

    #[test]
    fn get_arguments_gets_arguments() {
        let args: Vec<&str> = vec!["/something/bin/arma3", "foo", "bar", "-baz=boom"];
        let parser = ArgsParser::new(&args.into_iter().map(|s| s.to_string()).collect());

        let vec = parser.get_arguments();
        println!("VEC {} VEC", vec.join(","));
        assert!(vec.contains(&"foo".to_string()));
        assert!(vec.contains(&"bar".to_string()));
    }
}

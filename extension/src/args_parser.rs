use core::option::Option::Some;
use std::collections::HashMap;

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

        assert!(port.eq("2302"));
    }

    #[test]
    fn get_parameters_returns_empty_string_as_value_for_flags() {
        let args: Vec<&str> = vec!["/something/bin/arma3", "-foo", "bar"];
        let parser = ArgsParser::new(&args.into_iter().map(|s| s.to_string()).collect());
        let map = parser.get_options();
        let foo = map.get("foo").expect("no foo key o.O");

        assert_eq!(foo, "");
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

pub struct ArgsParser {
    args: Vec<String>,
}

impl ArgsParser {
    pub fn new(raw: &Vec<String>) -> ArgsParser {
        ArgsParser { args: raw.clone() }
    }

    /**
     * "options" are those arguments starting with a dash. they may have values.
     */
    pub fn get_options(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        let iter = self.args.iter();
        iter.for_each(|s| {
            if s.starts_with('-') {
                let mut bits = s.split("=");
                let mut k = match bits.next() {
                    Some(s) => s.to_string(),
                    None => panic!("this should never happen"),
                };
                let v = match bits.last() {
                    Some(v) => v.to_string(),
                    None => "".to_string(),
                };
                hash.insert(k.split_off(1), v);
            } else {
                // hash.insert(s.to_string().split_off(1), val);
            }
        });

        hash
    }

    /**
     * "arguments" are command line arguments without dash.
     */
    pub fn get_arguments(&self) -> Vec<String> {
        self.args
            .iter()
            .filter(|arg| !arg.starts_with("-"))
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}

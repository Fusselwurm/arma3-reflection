use core::option::Option::Some;
use std::collections::HashMap;

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
                    None => panic!("this can never happen"),
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

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
    pub fn get_options(&self) -> HashMap<String, Vec<String>> {
        let mut hash: HashMap<String, Vec<String>> = HashMap::new();
        let iter = self.args.iter();
        iter.for_each(|s| {
            if s.starts_with('-') {
                let mut bits = s.split("=");
                let mut k = bits.next().unwrap().to_string();
                let v = match bits.last() {
                    Some(v) => v.to_string(),
                    None => "".to_string()
                };
                let hash_key = k.split_off(1);
                match hash.get_mut(&hash_key)  {
                    Some(vec) => vec.push(v),
                    None => {
                        let mut vec = vec![];
                        vec.push(v);
                        hash.insert(hash_key, vec);
                    },
                };

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

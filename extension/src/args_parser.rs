use core::option::Option::Some;
use std::collections::HashMap;

pub struct ArgsParser {
    args: Vec<String>,
}

impl ArgsParser {
    pub fn new(args: Vec<String>) -> ArgsParser {
        ArgsParser { args: ArgsParser::split_at_newline(args.clone()) }
    }

    /*
        for some reason, on Windows at least, A3S passes additional params as-is (with newlines as separators).
        so:
         * A3S passes this: -x\n-y
         * env::args() returns them as one -x\n-y
         * Arma3 however recognizes -x -y as separate arguments
     */
    fn split_at_newline(args: Vec<String>) -> Vec<String> {
        args.iter()
            .flat_map(|s| {
                // oh no, I will *not* do the string delimiter dance. if it's in any way in quotes, i won't touch it
                if s.contains('"') || s.contains('\'') {
                    vec![s.clone()]
                } else {
                    s.split_ascii_whitespace().map(|s| {s.to_string()}).collect()
                }
            })
            .collect()
    }

    /*
     * "options" are those arguments starting with a dash. they may have values.
     */
    pub fn options(&self) -> HashMap<String, Vec<String>> {
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
     * "arguments" are raw command line arguments, delimited by space or line break (something with CR and NL)
     */
    pub fn arguments(&self) -> Vec<String> {
        self.args
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}

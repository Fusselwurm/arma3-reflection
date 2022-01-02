use core::option::Option::Some;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::string::String;
use crate::args_parser::ArgsParser;

#[cfg(test)]
mod startup_parameters_test {
    use crate::startup_parameters::get_params_file_contents;

    #[test]
    fn reads_test_param_file() {
        let str = get_params_file_contents("./resources/test_par_file.txt");
        assert_eq!(2, str.len());
        assert_eq!(Some(&"-mod=C:\\foo\\bar;".to_string()), str.get(0));
        assert_eq!(Some(&"-otherparam".to_string()), str.get(1));
    }
}


fn get_params_file_contents(filename: &str) -> Vec<String> {
    let mut contents = String::new();
    let path = env::current_dir();
    println!(
        "current dir {}",
        match path.ok().map(|p| p.to_str().map(|f| f.to_string())) {
            Some(str) => match str {
                Some(s) => s,
                None => String::new(),
            },
            None => "[NO PATH]".to_string(),
        }
    );
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("cannot open file {}: {}", filename, err),
    };
    match file.read_to_string(&mut contents) {
        Ok(size) => println!("read {} bytes from params file {}", size, filename),
        Err(err) => panic!("cannot read file {}: {}", filename, err),
    };

    contents
        .split('\n')
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub struct StartupParameters {
    env_args_parser: ArgsParser,
    parameter_file_args_parser: ArgsParser,
}

impl StartupParameters {
    pub fn new(env_args_parser: ArgsParser) -> StartupParameters {
        let parfile_args: Vec<String> = match env_args_parser.get_options().get("par") {
            Some(filename) => {
                get_params_file_contents(filename)
            }
            None => Vec::new()
        };

        let parameter_file_args_parser: ArgsParser = ArgsParser::new(&parfile_args);
        StartupParameters { env_args_parser, parameter_file_args_parser }
    }

    pub fn get_options(&self) -> HashMap<String, String> {
        let mut env_opts: HashMap<String, String> = self.env_args_parser.get_options();
        let par_opts: HashMap<String, String> = self.parameter_file_args_parser.get_options();

        env_opts.extend(par_opts);
        env_opts
    }

    pub fn get_arguments(&self) -> Vec<String> {
        let mut env_args = self.env_args_parser.get_arguments();
        let par_args = self.parameter_file_args_parser.get_arguments();

        env_args.extend(par_args);
        env_args
    }
}

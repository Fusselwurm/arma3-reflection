use crate::args_parser::ArgsParser;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

pub struct CommandLine {
    pub arguments: Vec<String>,
    pub options: HashMap<String, String>,
}

#[cfg(test)]
mod test_get_params_file_contents {
    use crate::get_command_line::get_params_file_contents;

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

pub fn get_command_line() -> CommandLine {
    let parser = ArgsParser::new(&get_command_line_args());

    CommandLine {
        arguments: parser.get_arguments(),
        options: parser.get_options(),
    }
}

fn get_command_line_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    let parser = ArgsParser::new(&args);

    match parser.get_options().get("par") {
        Some(filename) => {
            args.append(get_params_file_contents(filename).as_mut());
            args
        }
        None => args,
    }
}

pub fn get_command_line_raw() -> String {
    get_command_line_args().join(" ")
}

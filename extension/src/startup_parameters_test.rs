macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[cfg(test)]
mod get_params_file_contents_test {
    use crate::startup_parameters::get_params_file_contents;
    #[test]
    fn reads_test_param_file() {
        let str = get_params_file_contents("./resources/test_par_file.txt");
        assert_eq!(2, str.len());
        assert_eq!(Some(&"-mod=C:\\foo\\bar;".to_string()), str.get(0));
        assert_eq!(Some(&"-otherparam".to_string()), str.get(1));
    }
}

#[cfg(test)]
mod startup_parameters_test {
    use crate::args_parser::ArgsParser;
    use crate::startup_parameters::StartupParameters;

    #[test]
    fn reads_test_param_file() {
        let foo = vec_of_strings!["-par=./resources/test_par_file.txt"];
        let pars = StartupParameters::new(ArgsParser::new(&foo.to_vec()));

        for option in pars.get_options() {
            println!("{} => {}", option.0, option.1)
        }
        assert_eq!(3, pars.get_options().len());
        assert_eq!(pars.get_options().get("par"), Option::Some(&"./resources/test_par_file.txt".to_string()));
        assert_eq!(pars.get_options().get("mod"), Option::Some(&"C:\\foo\\bar;".to_string()));
        assert_eq!(pars.get_options().get("otherparam"), Option::Some(&"".to_string()));
        assert_eq!(0, pars.get_arguments().len());
    }

    #[test]
    #[should_panic]
    fn panics_on_par_file_not_found() {
        let foo = vec_of_strings!["-par=./resources/test_par_file_does_not_exist.txt"];
        StartupParameters::new(ArgsParser::new(&foo.to_vec()));
    }

    #[test]
    fn reads_arguments_from_cli() {
        let foo = vec_of_strings!["-par=./resources/test_par_file.txt", "foo"];
        let pars = StartupParameters::new(ArgsParser::new(&foo.to_vec()));
        assert_eq!(pars.get_arguments().len(), 1);
        assert_eq!(pars.get_arguments().get(0), Option::Some(&"foo".to_string()));
    }
}

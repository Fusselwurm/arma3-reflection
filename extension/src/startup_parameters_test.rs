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
    use arma_rs::IntoArma;
    use crate::args_parser::ArgsParser;
    use crate::startup_parameters::StartupParameters;

    #[test]
    fn reads_test_param_file() {
        let pars = StartupParameters::new(ArgsParser::new(vec_of_strings!["-par=./resources/test_par_file.txt"]));

        for option in pars.options() {
            println!("{} => {}", option.0, option.1.to_arma().to_string())
        }
        assert_eq!(3, pars.options().len());
        assert_eq!(pars.options().get("par").unwrap().to_vec(), vec![("./resources/test_par_file.txt".to_string())]);
        assert_eq!(pars.options().get("mod").unwrap().to_vec(), vec![("C:\\foo\\bar;".to_string())]);
        assert_eq!(pars.options().get("otherparam").unwrap().to_vec(), vec!["".to_string()]);
        assert_eq!(3, pars.arguments().len());
    }

    #[test]
    fn ignores_par_file_not_found() {
        let opts = StartupParameters::new(ArgsParser::new(vec_of_strings!["-par=./resources/test_par_file_does_not_exist.txt"])).options();
        assert_eq!(1, opts.len());
    }

    #[test]
    fn reads_arguments_from_cli() {
        let pars = StartupParameters::new(ArgsParser::new(vec_of_strings!["-par=./resources/test_par_file.txt", "foo"]));
        assert_eq!(pars.arguments().len(), 4);
        assert_eq!(pars.arguments().get(1), Option::Some(&"foo".to_string()));
    }
}

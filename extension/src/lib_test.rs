#[cfg(test)]
mod tests {
    use crate::init;

    #[test]
    fn get_option_first_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_option_first", Some(vec!["foo_i_dont_exist".to_string()])) };
        assert_eq!(output, "");
    }

    #[test]
    fn get_option_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_option", Some(vec!["foo_i_dont_exist".to_string()])) };
        assert_eq!(output, "[]");
    }

    #[test]
    fn get_argument_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_argument", Some(vec!["999".to_string()])) };
        assert_eq!(output, "");
    }

    #[test]
    fn get_cmdline_raw_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_commandline_raw", Some(vec!["0".to_string()])) };
        assert_ne!(output, "");
    }

    #[test]
    fn get_string_of_len_works() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_string_of_len", None) };
        assert_eq!(output, "");
        let (output, _) = unsafe { extension.call("get_string_of_len", Some(vec!["22".to_string()])) };
        assert_eq!(output, "xxxxxxxxxxxxxxxxxxxxxx");

        let (output, _) = unsafe { extension.call("get_string_of_len", Some(vec!["-5".to_string()])) };
        assert_eq!(output, "");
    }
}

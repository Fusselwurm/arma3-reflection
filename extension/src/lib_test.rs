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
    fn get_argument_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_argument", Some(vec!["999".to_string()])) };
        assert_eq!(output, "");
    }

    #[test]
    fn get_cmdline_raw_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("get_cmdline_raw", None) };
        assert_ne!(output, "");
    }
}

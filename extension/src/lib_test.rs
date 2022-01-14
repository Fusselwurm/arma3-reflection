#[cfg(test)]
mod tests {
    use crate::init;

    #[test]
    fn get_option_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("commandline:option", Some(vec!["foo_i_dont_exist".to_string()])) };
        assert_eq!(output, "[]");
    }

    #[test]
    fn get_argument_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("commandline:argument", Some(vec!["999".to_string()])) };
        assert_eq!(output, "");
    }

    #[test]
    fn get_cmdline_raw_does_not_fail() {
        let extension = init().testing();
        let (output, _) = unsafe { extension.call("commandline:raw", Some(vec!["0".to_string()])) };
        assert_ne!(output, "");
    }
}

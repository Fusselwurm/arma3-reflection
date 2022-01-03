#[cfg(test)]
mod tests {
    use crate::{get_argument, get_option_first, get_cmdline_raw, init};

    #[test]
    fn init_exists() {
        init();
    }

    #[test]
    fn get_parameter_does_not_fail() {
        assert!(get_option_first(&("foo").to_string()).eq(""));
    }

    #[test]
    fn get_argument_does_not_fail() {
        assert!(get_argument(999).eq(""));
    }

    #[test]
    fn get_cmdline_raw_does_not_fail() {
        assert_ne!(get_cmdline_raw(), "".to_string());
    }
}

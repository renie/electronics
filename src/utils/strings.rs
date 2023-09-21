pub fn remove_crlf(mut input: String) -> String {
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);

    return input;
}


#[cfg(test)]
mod tests {
    use super::remove_crlf;

    #[test]
    fn remove_crlf_n_to_be_removed() {
        assert_eq!("foo", remove_crlf(String::from("foo\n")));
    }

    #[test]
    fn remove_crlf_r_to_be_removed() {
        assert_eq!("foo", remove_crlf(String::from("foo\r")));
    }

    #[test]
    fn remove_crlf_rn_to_be_removed() {
        assert_eq!("foo", remove_crlf(String::from("foo\r\n")));
    }

    #[test]
    fn remove_crlf_nr_to_be_removed() {
        assert_eq!("foo", remove_crlf(String::from("foo\n\r")));
    }

    #[test]
    fn remove_crlf_nothing_to_be_removed() {
        assert_eq!("foo", remove_crlf(String::from("foo")));
    }
}

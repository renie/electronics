pub fn remove_crlf(mut input: String) -> String {
    let len = input.trim_end_matches(&['\r', '\n'][..]).len();
    input.truncate(len);

    return input;
}

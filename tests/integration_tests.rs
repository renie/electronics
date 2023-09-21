use electronics::run_with_params;

#[test]
fn gets_valid_content() -> Result<(), String> {
    match run_with_params(String::from("sample.json")) {
        Ok(_) => Ok(()),
        Err(message) => Err(format!("Running with filename as paramater. Should go all good. Error raised: {} ", message))
    }
}

#[test]
fn gets_content_invalid_file() -> Result<(), String> {
    match run_with_params(String::from("bla.txt")) {
        Ok(_) => Err(format!("Running with filename as paramater. Should NOT go all good.")),
        Err(_) => Ok(())
    }
}

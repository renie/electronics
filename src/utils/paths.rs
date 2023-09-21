use std::path::Path;

pub fn check_path_validity(path: &String) -> Result<bool, &'static str> {
    let slice_of_path = Path::new(path);
    if !slice_of_path.is_file() {
        return Err("File not found.");
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::check_path_validity;

    #[test]
    fn check_path_validity_exist() -> Result<(), String> {
        match check_path_validity(&String::from("sample.json")) {
            Ok(_) => Ok(()),
            Err(message) => Err(format!("File should exist. But it doesn't. Error raised: {}", message))
        }
    }

    #[test]
    fn check_path_validity_doesnot_exist() -> Result<(), String> {
        match check_path_validity(&String::from("bla.txt")) {
            Ok(contents) => Err(format!("File should NOT exist. But IT DOES. Contents returned: {}", contents)),
            Err(_) => Ok(())
        }
    }
}

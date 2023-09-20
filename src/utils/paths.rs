use std::path::Path;

pub fn check_path_validity(path: &String) -> Result<bool, &'static str> {
    let slice_of_path = Path::new(path);
    if !slice_of_path.is_file() {
        return Err("File not found.");
    }
    Ok(true)
}


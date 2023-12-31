use std::fs;
mod utils;

pub fn run() -> Result<String, &'static str> {    
    let filename = match get_circuit_filename_from_user() {
        Ok(name) => name,
        Err(message) => {
            return Err(message)
        }
    };
    run_with_params(filename)
}

pub fn run_with_params(filename: String) -> Result<String, &'static str> {
    get_file_contents(filename)
}

fn get_circuit_filename_from_user() -> Result<String, &'static str> {
    match utils::user_interaction::get_user_response_for(&"Where is your circuit file?") {
        Ok(file_name_input) => Ok(file_name_input),
        Err(message) => Err(message)
    }
}

fn get_file_contents(filename: String) -> Result<String, &'static str> {
    match utils::paths::check_path_validity(&filename) {
        Ok(_) => {
            let contents = fs::read_to_string(filename)
                .expect("Something went wrong reading the file. Exiting..."); 
            return Ok(contents)
        },
        Err(message) => Err(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_contents_exist() -> Result<(), String> {
        match get_file_contents(String::from("sample.json")) {
            Ok(_) => Ok(()),
            Err(message) => Err(format!("File should exist. But it doesn't. Error raised: {}", message))
        }
    }   

    #[test]
    fn get_file_contents_doesnot_exist() -> Result<(), String> {
        match get_file_contents(String::from("bla.txt")) {
            Ok(contents) => Err(format!("File should NOT exist. But IT DOES. Contents returned: {}", contents)),
            Err(_) => Ok(())
        }
    }    
}

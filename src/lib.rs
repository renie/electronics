use std::fs;

mod utils;
mod models;

use models::gate::Gate;

pub fn run() -> Result<Vec<Gate>, &'static str> {
    let filename = match get_circuit_filename_from_user() {
        Ok(name) => name,
        Err(message) => {
            return Err(message)
        }
    };
    run_with_params(filename)
}

pub fn run_with_params(filename: String) -> Result<Vec<Gate>, &'static str> {
    match get_file_contents(filename) {
        Ok(contents) => match Gate::parse_json_list(contents.as_str()) {
            Ok(list) => {
                println!("Circuit Result: {}", Gate::process_circuit_result(list.clone()));
                Ok(list)
            },
            Err(e) => {
                dbg!(e);
                Err("Error while parsing JSON.")
            }
        },
        Err(message) => Err(message)
    }
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

    // TODO: Still don' t know how to test get_circuit_filename_from_user function
    // The test will have to interact with read_line, that need to be solved to be able to test
    // that function
}

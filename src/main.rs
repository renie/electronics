use std::io::{self};
use std::{fs, process};
mod utils;


// TODO: add tests after refactoring
fn main() -> io::Result<()> {

    let file_name = match utils::user_interaction::get_user_response_for(&"Where is your circuit file?") {
        Ok(file_name_input) => file_name_input,
        Err(message) => {
            eprintln!("{} \nExiting...", message);
            process::exit(1)
        } 
    };

    match utils::paths::check_path_validity(&file_name) {
        Ok(_) => {
            let contents = fs::read_to_string(file_name)
                .expect("Something went wrong reading the file. Exiting..."); 
            println!("File {}", contents);
            Ok(())
        },
        Err(message) => {
            eprintln!("{} \nExiting...", message);
            process::exit(1)
        } 
    }
}

use std::io::{self};
use std::{fs, process};
use std::path::Path;

// TODO: add tests after refactoring
fn main() -> io::Result<()> {
    // TODO: create getUserResponseFor(question:String)->Result<String (trimmed),Err (if not informed)>
    println!("Where is your circuit file?");
    let mut user_input  = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut user_input);
    // TODO: create removeCRLF(&mut self)
    let len = user_input.trim_end_matches(&['\r', '\n'][..]).len();
    user_input.truncate(len);


    if user_input.is_empty() {
        eprintln!("No file informed. Exiting...");
        process::exit(1)
    }


    // TODO: create checkPathValidity(path:String):Result<Ok(),Err (File not found)>
    let path = Path::new(&user_input);
    if !path.is_file() {
        eprintln!("File not found. Exiting...");
        process::exit(1)
    }


    let contents = fs::read_to_string(user_input)
        .expect("Something went wrong reading the file. Exiting..."); 


    println!("File {}", contents);

    Ok(())
}

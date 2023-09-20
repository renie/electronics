use std::io::{self};
use crate::utils::strings::remove_crlf;

pub fn get_user_response_for(question: &str) -> Result<String, &'static str> {
    println!("{}", question);
    let mut user_input  = String::new();
    let stdin = io::stdin();
    let _ = stdin.read_line(&mut user_input);
    
    let trimmed_input = remove_crlf(user_input);  
    if trimmed_input.is_empty() {
        return Err("File not informed.");
    }
    
    Ok(trimmed_input)
}

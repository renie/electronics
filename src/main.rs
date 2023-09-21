use std::process;
use electronics::run;

// TODO: add tests after refactoring
fn main() {
    match run() {
        Ok(contents) => {
            println!("Gate id {} has name {}", contents.last().unwrap().id, contents.last().unwrap().name);
        },
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    };
}

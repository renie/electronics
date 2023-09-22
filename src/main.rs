use std::process;
use electronics::run;

// TODO: add tests after refactoring
fn main() {
    match run() {
        Ok(contents) => {
            println!("Result: {}", contents.first().unwrap().process_result());
            println!("Result: {}", contents.last().unwrap().process_result());
            dbg!(contents);
        },
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    };
}

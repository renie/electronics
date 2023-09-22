use std::process;
use electronics::run;

// TODO: add tests after refactoring
fn main() {
    match run() {
        Ok(contents) => {
            dbg!(contents);
        },
        Err(message) => {
            eprintln!("{}", message);
            process::exit(1);
        }
    };
}

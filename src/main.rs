#[macro_use]
extern crate scan_rules;
mod util;

fn main() {
    let mut history = util::initialize_vector();

    loop {
        print!("rustshell@rustshell:$ ");
        let result = try_readln! {
            (let command: String) => (command)
        };
    }
    
}



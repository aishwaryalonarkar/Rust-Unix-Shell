mod util;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    let mut history = util::initialize_vector();
    let command_history:String = String::from("history");
    let mut command:String = String::new();

    loop {
        print!("rustshell@rustshell:$ ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut command).unwrap();

        history = util::add_command_to_history(history, command.clone());
        
        //println!("{}", command);
        if command.eq(&command_history) {
            history = util::list_history(history);
        } else {
            println!("Invalid command");
        }
        command.clear();
    }
    
}



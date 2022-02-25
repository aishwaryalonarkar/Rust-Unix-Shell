mod util;
use std::io::Write;

fn main() {
    let mut history = util::initialize_vector();
    let command_history:String = String::from("cmd_history");
    let command_quit: String = String::from("quit");
    let mut command:String = String::new();

    loop {
        print!("rustshell@rustshell:~$ ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut command).unwrap();

        command = String::from(&command[..command.len() - 1]);

        history = util::add_command_to_history(history, command.clone());
        
        if command == command_history {
            history = util::list_history(history);
        } else if command == command_quit {
            println!("Quitting");
            break;
        } else {
            println!("Invalid command");
        }

        command.clear();
    }
}



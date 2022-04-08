mod util;
mod rmallexn;
use std::io::Write;

fn main() {
    let mut history = util::initialize_vector();
    let command_history:String = String::from("cmd_history");
    let command_quit: String = String::from("quit");
    let mut command:String = String::new();

    // Retrieve the history commands if any before starting the shell
    history = util::retrieve_history(history);

    loop {
        print!("rustshell@rustshell:~$ ");
        // Flushes the output to stdout as prints without new line are buffered and we have 
        // to explicitly flush the buffer.
        std::io::stdout().flush().unwrap();

        // Reads the input from the command line
        std::io::stdin().read_line(&mut command).unwrap();

        // Reading input from command line adds a new line character at the end.
        // Copying everything back to command except the new line character to match 
        // the strings later with the commands.
        command = String::from(&command[..command.len() - 1]);

        // If no input is entered and only return is pressed, it does not track those inputs
        if command.len() == 0 {
            command.clear();
            continue;
        }

        // Every command whether valid or invalid is added to the history list
        history = util::add_command_to_history(history, command.clone());
        
        if command == command_history {
            history = util::list_history(history);
        } else if command.starts_with("rmallexn"){
            rmallexn::rmxn(command.clone());
        }
        else if command == command_quit {
            println!("Quitting");
            util::write_results_in_file(command_history, history.clone());
            break;
        } else {
            println!("Invalid command");
        }

        // Clear the command variable in order to start fresh for the next iteration
        command.clear();
    }
}

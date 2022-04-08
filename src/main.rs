mod util;
mod ls_tree;
mod ls;
mod ls_color;
use std::io::Write;
use std::path::Path;

// use std::fs;

fn main() {
    let mut history = util::initialize_vector();
    let command_history:String = String::from("cmd_history");
    let command_quit: String = String::from("quit");
    let command_ls: String = String::from("ls"); //listDir
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
        

        // splitting for listDir Options.
        command = command.trim().to_string();
        let g = command.split(" ");
        let vec: Vec<&str> = g.collect();
        let mut path = vec[vec.len()-1];


        if command == command_history {
            history = util::list_history(history);
        } 
        else if vec[0] == command_ls {
            
            // Set default path ./ if no path input.
            if path == "-a" || path == "-tree" || path == "-l" || path =="-color" || path == command_ls {
                path = "";
            }
            // Check if input path exist
            if !Path::new(path).exists() && path != "" {
                println!("Path does not exist -> {}", path);
                
            }
            else {
                // Remove path from command 
                let mut command_t = command.replace(path, "");
                // trim all whitespaces 
                command_t = command_t.replace(" ", "");
                match command_t.as_str() {
                    "ls-a" => ls_tree::list_all(path.to_string()),
                    "ls-tree" => ls_tree::tree_display(path.to_string()),
                    "ls-l-color" => ls_color::ls_color_main(path.to_string()),
                    "ls-color-l" => ls_color::ls_color_main(path.to_string()),
                    "ls-l" => ls::ls_main(path.to_string()),
                    _=>{
                        println!("Invalid option");
                        println!("listDir [-l] [-a] [-tree] [-color] <directory>");
                    }
                }
            }

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

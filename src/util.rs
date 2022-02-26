use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate json;

// Initialize the vector to store the list of commands entered on the shell
pub fn initialize_vector() -> Vec<String> {
    let vec = Vec::new();

    vec
}

// Add command to the history vector
pub fn add_command_to_history(mut history:Vec<String>, command:String) -> Vec<String> {
    history.push(command);

    history
}

// Print the commands present in the history vector
pub fn list_history(history:Vec<String>) -> Vec<String> {
    for i in 0..history.len() {
        println!("\t{0} {1}", i + 1, history[i]);
    }

    history
}

// Write the cmd_history to a file
pub fn write_results_in_file(key: String, value: Vec<String>) {
    let file_name = "log.txt";
    let path = Path::new(file_name);
    let display = path.display();

    // Check the existence of the file in the current directory. If exists, read the file, extract the 
    // JSON from the file and create a file with write only permission and write the updated data
    // into the file. If file does not exists, create the file and write the data into it.
    let exists = Path::new(file_name).exists();

     // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
    if exists {
        let mut file = match File::options()
            .read(true)
            .write(true)
            .open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("")
        }

        let mut parsed_data = json::parse(&data).unwrap();
        parsed_data[key] = value.into();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(json::stringify(parsed_data).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    } else {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // https://docs.rs/json/latest/json/
        let mut data = json::JsonValue::new_object();
        data[key] = value.into();

        match file.write_all(json::stringify(data).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}

pub fn retrieve_history(mut history:Vec<String>) -> Vec<String> {
    let file_name = "log.txt";
    let path = Path::new(file_name);
    let display = path.display();

    // Check for file existence
    let exists = Path::new(file_name).exists();

    if exists {
        // Open the file with read permission
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the contents of the file and convert it into the String format
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("")
        }

        // Convert the extracted data from String to JSON type and then extract the value
        // of key "cmd_history"
        let data = json::parse(&data).unwrap();
        let cmd_history = &data["cmd_history"];

        // Add every entry into the JSON array into the history vector using stringify.
        for i in 0..cmd_history.len() {
            history = add_command_to_history(history, json::stringify(cmd_history[i].clone()));
             // Rewrite every entries into the history vector such that double quotes ("") are not
            // copied into the history vector. e.g. "ls" is copied as ls into the vector
            history[i] = String::from(&history[i][1..history[i].len() - 1]);
        }
    }

    history
}
 
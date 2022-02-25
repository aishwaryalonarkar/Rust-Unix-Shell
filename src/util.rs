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

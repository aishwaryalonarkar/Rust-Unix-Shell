mod util;

fn main() {
    let mut history = util::initialize_vector();
    history = util::add_command_to_history(history, String::from("ls"));
    history = util::add_command_to_history(history, String::from("cd .."));
    history = util::add_command_to_history(history, String::from("mkdir"));
    history = util::add_command_to_history(history, String::from("touch abc.txt"));
    history = util::list_history(history);
}



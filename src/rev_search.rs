extern crate termion;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::rev_search::termion::cursor::DetectCursorPos;
use crate::util;

fn search_substring(history: Vec<String>, command:String, mut index:usize) -> usize {
    while index < history.len() {
        if !history[history.len() - 1 - index].contains(command.as_str()) {
            index += 1;
            continue;
        }

        print!("\t -> {}", history[history.len() - 1 - index]);
        break;
    }

    index 
}


pub fn rev_search(history:Vec<String>) -> Vec<String> {
    // https://github.com/redox-os/termion/blob/master/examples/keys.rsm
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    let mut command = String::new();
    let mut index:usize = 0;

    let cursor_pos = stdout.cursor_pos().unwrap();
    let (mut x, y) = cursor_pos;

    write!(stdout, "{}{}", termion::cursor::Goto(x,y), 
           termion::clear::CurrentLine).unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(command_line) => {
                if command_line == '\n' {
                    write!(stdout, "{}{}", termion::cursor::Goto(1,y), 
                           termion::clear::CurrentLine).unwrap();
                    println!("{}", history[history.len() - 1 - index]);
                    write!(stdout, "{}{}", termion::cursor::Goto(1, y), 
                       termion::clear::AfterCursor).unwrap();
                    break;
                }

                write!(stdout, "{}{}", termion::cursor::Goto(x + 1, y), 
                       termion::clear::AfterCursor).unwrap(); 
                print!("{}", command_line);
                command.push(command_line);
                index = search_substring(history.clone(), command.clone(), index);
                x += 1;
            },
            Key::Backspace => {
                write!(stdout, "{}{}", termion::cursor::Goto(x, y), 
                       termion::clear::AfterCursor).unwrap();
                if x > 1 {
                    x -= 1;
                    command = command[0..command.len() - 1].to_string();
                }
                
                index = 0;
            },
            Key::Ctrl('c') => break,
            _ => {
                
            }
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();

    let (x, y) = stdout.cursor_pos().unwrap();

    if x == 1 {
        util::dispatch_function_helper(history.clone(), history[history.len() - 1 - index].clone());
    }

    history
}
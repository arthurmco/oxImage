mod picture;

use std::io;
use std::io::Write;
use picture::*;

fn draw_command_line(ps1: &str) -> Option<String> {
    print!("{}", ps1);
    io::stdout().flush().expect("Error while flushing stdout");

    let mut command = String::new();
    if io::stdin().read_line(&mut command).is_err() {
        return None;
    };

    command = String::from(command.trim());

    if command.len() == 0 {
        return None;
    }

    return Some(command);
}

fn main() {
    println!("oxImage v0.1");
    println!("Copyright (c) 2017 Arthur M");
    println!("Licensed under MIT License\n");

    let root = ImageCollectionList::create("root");
    let cwd = root;
    loop {

        let cmd = match draw_command_line("> ") {
            Some(command) => command,
            None => continue
        };

        if cmd == "quit" {
            println!("Exiting...\n");
            break;
        } else if cmd == "pwd" {
            println!(" {}/ \n", cwd.get_display_name());
        } else {
            println!(" Command {} does not exist\n", cmd);
        }

    }


}

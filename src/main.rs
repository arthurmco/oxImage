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
    println!("Licensed under MIT License");
    println!("Type 'help' for help\n");

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
        } else if cmd == "ls" {
            println!("{} items\n", cwd.count());

            let it = cwd.iterator();
            for item in it {
                match item {
                    &ImageCollectionItem::ImageItem(ref img) => println!("{}, image", img.get_display_name()),
                    &ImageCollectionItem::ImageList(ref ilst) => println!("{}, collection", ilst.get_display_name())
                };
            }

        } else if cmd == "help" {
            println!("quit: Exit");
            println!("pwd: Show actual collection you're in");
            println!("ls: List collection content");
            println!("");
        } else {
            println!(" Command {} does not exist\n", cmd);
        }

    }


}

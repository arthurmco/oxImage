mod picture;

use std::io;
use std::env;
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
    let mut cwd = root;
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
        } else if cmd == "add" {
            print!("Please type file name: ");
            io::stdout().flush().expect("Error while flushing stdout");

            let mut fname = String::new();
            io::stdin().read_line(&mut fname).expect(
                "Error while opening stdin"
            );

            fname = String::from(fname.trim());

            println!("opening '{}' from '{}'", fname, env::current_dir().unwrap().display());

            let img = match picture::BasicImageOpener::open(&fname) {
                Ok(i) => i,
                Err(e) => {
                    match e {
                        picture::ImageError::FormatError(err) => {
                            eprintln!("error: image isn't correctly formatted: {}", err);
                        },
                        picture::ImageError::DimensionError => {
                            eprintln!("error: image is too small or too large");
                        },
                        picture::ImageError::UnsupportedError(err) => {
                            eprintln!("error: image format unsupported: {}", err);
                        },
                        picture::ImageError::UnsupportedColor(_) => {
                            eprintln!("error: image color format unsupported");
                        },
                        picture::ImageError::NotEnoughData => {
                            eprintln!("error: not enough data for decoding");
                        },
                        picture::ImageError::IoError(err) => {
                            eprintln!("error: I/O error: {}", err);
                        },
                        picture::ImageError::ImageEnd => {
                            eprintln!("error: unexpected eof");
                        },
                    };
                    continue;
                }
            };

            println!("opened {}, dimensions {}x{}", img.path, img.get_width(), img.get_height());
            cwd.add_element(ImageCollectionItem::ImageItem(img));
            println!("image added\n");

        } else if cmd == "help" {
            println!("quit: Exit");
            println!("add: Add an image");
            println!("pwd: Show actual collection you're in");
            println!("ls: List collection content");
            println!("");
        } else {
            println!(" Command {} does not exist\n", cmd);
        }

    }


}

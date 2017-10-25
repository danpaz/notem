extern crate notem;

use std::env;
use std::process;
use std::io::{stderr, Write};

use notem::config::Config;
use notem::list::list_notes;
use notem::open;

fn usage() {
    println!("usage:
notem <some subject>
    Start a new note with the given <some subject> in the file name.
notem {{--list|-l}}
    List all notes.");
}

fn main() {
    let mut args = env::args();
    match env::args().skip(1).next() {
        Some(arg) => {
            match arg.as_str() {
                "--list" | "-l" => {
                    let config = Config::new(args).unwrap();
                    let path = config.path;
                    if let Err(err) = list_notes(&path) {
                        writeln!(stderr(), "An error occurred: {}", err).ok();
                        process::exit(1);
                    };
                },
                "--open" | "-o" | _ => {
                    args.next();
                    let config = Config::new(args).unwrap();
                    let Config { editor, mut path, filename } = config;
                    path.push(filename);
                    if let Err(err) = open::that(editor, path) {
                        writeln!(stderr(), "An error occurred: {}", err).ok();
                        process::exit(1);
                    };
                },
            };
        },
        None => {
            usage();
            process::exit(1);
        }
    };
}

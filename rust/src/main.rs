extern crate notem;

use std::env;
use std::process;

use notem::config::Config;
use notem::open;
use notem::dir;

fn usage() {
    println!("usage:
notem <some subject>
    Start a new note with the given <some subject> in the file name.
notem {{--list|-l}}
    List all notes.
notem {{--search|-s}} <some subject>
    Search notes by file name.");
}

fn main() {
    let mut args = env::args();
    args.next();
    match env::args().skip(1).next() {
        Some(arg) => {
            match arg.as_str() {
                "--list" | "-l" => {
                    let config = Config::new(args).unwrap();
                    dir::list_notes(&config.path);
                },
                "--search" | "-s" => {
                    args.next();
                    let rest = env::args().skip(2).collect();
                    let config = Config::new(args).unwrap();
                    dir::search_notes(&config.path, rest);
                },
                "--open" | "-o" => {
                    args.next();
                    let config = Config::new(args).unwrap();
                    let Config { editor, mut path, filename } = config;
                    path.push(filename);
                    open::that(editor, path);
                },
                _ => {
                    let config = Config::new(args).unwrap();
                    let Config { editor, mut path, filename } = config;
                    path.push(filename);
                    open::that(editor, path);
                }
            };
        },
        None => {
            usage();
            process::exit(1);
        }
    };
}

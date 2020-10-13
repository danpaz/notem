extern crate notem;

use std::env;
use std::process;

use notem::config::Config;
use notem::dir;
use notem::open;

fn usage() {
    println!(
        "Usage:
\tnotem <some subject>
\t    Start a new note with the given <some subject> in the file name.
\tnotem {{--list|-l}}
\t    List all notes.
\tnotem {{--open|-o}} <some subject>
\t    Open a note.
\tnotem {{--search|-s}} <some subject>
\t    Search notes by file name."
    );
}

fn run(args: Vec<String>) -> Result<(), String> {
    let first = args[1].as_str();
    let rest = &args[2..];
    let config = Config::new(rest).unwrap();

    match first {
        "--list" | "-l" => {
            dir::list_notes(&config.path).map_err(|e| format!("Could not list files due to {}", e))
        }
        "--print" | "-p" => dir::print_notes(&config.path, rest)
            .map_err(|e| format!("Could not print files due to {}", e)),
        "--search" | "-s" => dir::search_notes(&config.path, rest)
            .map_err(|e| format!("Could not search files due to {}", e)),
        "--open" | "-o" => {
            let Config {
                editor,
                mut path,
                filename,
            } = config;
            path.push(filename);
            match open::that(editor, path) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Could not open file due to {}", e)),
            }
        }
        // If no command line arguments are given, just open a new file
        _ => {
            let Config {
                editor,
                mut path,
                filename,
            } = Config::new(&args[1..]).unwrap();
            path.push(filename);
            match open::that(editor, path) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Could not open file due to {}", e)),
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        process::exit(1);
    } else {
        run(args).unwrap();
    }
}

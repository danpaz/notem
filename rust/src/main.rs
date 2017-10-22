extern crate notem;

use std::io::{stderr, Write};
use std::env;
use std::process;

use notem::config::Config;
use notem::open;

fn main() {
    let config = Config::new(env::args()).unwrap();
    if let Err(err) = open::that(&config.editor, &config.path) {
        writeln!(stderr(), "An error occurred: {}", err).ok();
        process::exit(3);
    }
}

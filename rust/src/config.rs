use std::env;
use std::path::PathBuf;
use chrono::prelude::*;
use which::which;

pub struct Config {
    pub editor: PathBuf,
    pub path: PathBuf,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let editor = try!(choose_editor());
        let mut path = try!(get_path());
        let filename = get_filename(args.collect());
        path.push(filename);

        Ok(Config {
          editor: editor,
          path: path,
        })
    }
}

fn choose_editor() -> Result<PathBuf, &'static str> {
    match env::var_os("EDITOR") {
        Some(val) => {
            if which(&val).is_ok() {
                Ok(PathBuf::from(val))
            } else {
                choose_fallback_editor()
            }
        },
        None => choose_fallback_editor()
    }
}

fn choose_fallback_editor() -> Result<PathBuf, &'static str> {
    for program in &["atom", "subl", "mate", "edit", "vim", "vi"] {
        if which(program).is_ok() {
            return Ok(PathBuf::from(program));
        }
    }
    Err("Could not find any editor")
}

fn get_path() -> Result<PathBuf, &'static str> {
    match env::var_os("NOTEM_PATH") {
        Some(val) => {
            Ok(PathBuf::from(val))
        },
        None => {
            // Default path is ~/notes
            let mut path = env::home_dir().unwrap();
            path.push("notes");
            Ok(path)
        }
    }

}

// prepend date, lowercase, dasherize
fn get_filename(args: Vec<String>) -> PathBuf {
    let date: DateTime<Local> = Local::now();
    let mut terms: Vec<String> = vec![
        date.year().to_string(),
        date.month().to_string(),
        date.day().to_string()
    ];
    terms.extend(args);
    // Results in "2017-10-12-args-go-here"
    let filename = terms
        .iter()
        .map(|x| x.to_lowercase())
        .collect::<Vec<String>>()
        .join("-");

    PathBuf::from(filename)
}

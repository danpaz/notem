use chrono::prelude::*;
use std::env;
use std::path::PathBuf;
use which::which;

pub struct Config {
    pub editor: PathBuf,
    pub path: PathBuf,
    pub filename: PathBuf,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, String> {
        let editor = choose_editor()?;
        let path = get_path()?;
        let filename = get_filename(args);

        Ok(Self {
            editor,
            path,
            filename,
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
        }
        None => choose_fallback_editor(),
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
        Some(val) => Ok(PathBuf::from(val)),
        None => {
            // Default path is ~/notes
            let mut path = dirs::home_dir().unwrap();
            path.push("notes");
            Ok(path)
        }
    }
}

// prepend date, lowercase, dasherize
fn get_filename(args: &[String]) -> PathBuf {
    let date: DateTime<Local> = Local::now();
    let mut terms: Vec<String> = vec![
        date.year().to_string(),
        date.month().to_string(),
        date.day().to_string(),
    ];
    terms.extend_from_slice(args);
    // Results in "2017-10-12-args-go-here"
    let filename = terms
        .iter()
        .map(|x| x.to_lowercase())
        .collect::<Vec<String>>()
        .join("-");

    PathBuf::from(filename)
}

#[test]
fn test_get_filename() {
    let args: Vec<String> = vec!["this".into(), "is".into(), "a".into(), "subject".into()];
    let now = Local::now();

    assert_eq!(
        get_filename(&args[..]).to_str().unwrap(),
        format!(
            "{}-{}-{}-{}",
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
            "this-is-a-subject"
        )
    );
}

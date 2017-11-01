use std::fs;
use std::path::PathBuf;
use std::io;

pub fn search_notes(path: &PathBuf, args: Vec<String>) -> io::Result<()> {
  let search = args
      .iter()
      .map(|x| x.to_lowercase())
      .collect::<Vec<String>>()
      .join("-");

  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let file_name = entry.file_name().into_string().unwrap();
    if file_name.contains(&search) {
      println!("{}", entry.path().display());
    }
  };
  Ok(())
}

use std::fs;
use std::path::PathBuf;
use std::io;

pub fn list_notes(path: &PathBuf) -> io::Result<()> {
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    println!("{}", entry.path().display());
  }
  Ok(())
}

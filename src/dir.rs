use std::fs;
use std::path::PathBuf;
use std::io;

pub fn list_notes(path: &PathBuf) -> io::Result<()> {
  read_notes_dir(path).iter()
    .for_each(|entry| {
      println!("{}", entry.path().display())
    });
  Ok(())
}

pub fn search_notes(path: &PathBuf, args: Vec<String>) -> io::Result<()> {
  let search = args
      .iter()
      .map(|x| x.to_lowercase())
      .collect::<Vec<String>>()
      .join("-");

  read_notes_dir(path).iter()
    .filter(|entry| {
      entry.file_name().into_string().unwrap().contains(&search)
    })
    .for_each(|entry| {
      println!("{}", entry.path().display())
    });

  Ok(())
}

fn read_notes_dir(path: &PathBuf) -> Vec<fs::DirEntry> {
  fs::read_dir(path).unwrap()
    .map(|e| e.unwrap())
    .collect()
}

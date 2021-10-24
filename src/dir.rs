use std::fs;
use std::io;
use std::path::Path;

fn search_term(args: &[String]) -> String {
    args.iter()
        .map(|x| x.to_lowercase())
        .collect::<Vec<String>>()
        .join("-")
}

pub fn list_notes(path: &Path) -> io::Result<()> {
    read_notes_dir(path)
        .iter()
        .for_each(|entry| println!("{}", entry.path().display()));
    Ok(())
}

pub fn search_notes(path: &Path, args: &[String]) -> io::Result<()> {
    let search = search_term(args);

    read_notes_dir(path)
        .iter()
        .filter(|entry| entry.file_name().into_string().unwrap().contains(&search))
        .for_each(|entry| println!("{}", entry.path().display()));

    Ok(())
}

pub fn print_notes(path: &Path, args: &[String]) -> io::Result<()> {
    let search = search_term(args);
    read_notes_dir(path)
        .iter()
        .filter(|entry| entry.file_name().into_string().unwrap().contains(&search))
        .for_each(|entry| {
            let contents =
                fs::read_to_string(entry.path()).expect("Something went wrong reading the file");
            println!("{}", contents);
        });

    Ok(())
}

fn read_notes_dir(path: &Path) -> Vec<fs::DirEntry> {
    fs::read_dir(path).unwrap().map(|e| e.unwrap()).collect()
}

use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use tempfile::tempfile;

fn rename(pattern: &str, replace: &str, root_path: &str, test: &bool) -> std::io::Result<()> {
    let mut tmp_files: Vec<(File, String)> = Vec::new();

    match Regex::new(pattern) {
        Err(_) => {
            println!("Invalid Regex :(");
            return Ok(());
        }
        Ok(regex) => {
            // Iterate over the files in the current directory
            for entry in fs::read_dir(root_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(stem) = path.file_stem() {
                        if !stem.to_string_lossy().starts_with('.') {
                            let path_string = &path.display().to_string();
                            if let Some(_matched) = regex.find(path_string) {
                                let result = regex.replace_all(path_string, replace);

                                let new_name = format!("{}", result.trim());
                                println!("{} -> {}", path.display().to_string(), new_name);

                                if *test != true {
                                    let mut temp_file = tempfile()?;
                                    let content = fs::read(&path)?;
                                    temp_file.write_all(&content)?;
                                    temp_file.seek(SeekFrom::Start(0)).unwrap();
                                    tmp_files.push((temp_file, new_name));
                                    fs::remove_file(&path)?;
                                }
                            }
                        }
                    }
                }
            }

            // Create new files from the temporary files
            for (mut tmp_file, new_file_name) in tmp_files.into_iter() {
                let new_path = Path::new(&new_file_name);
                let mut new_file = File::create(&new_path)?;
                let mut buffer = Vec::new();
                tmp_file.read_to_end(&mut buffer)?;
                new_file.write_all(&buffer)?;
            }
            Ok(())
        }
    }
}

fn main() {
    let matches = Command::new("rename-enum")
        .version("1.0")
        .about("rename all files by enumerating")
        .arg(
            Arg::new("pattern")
                .help("Search pattern")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("substitution")
                .help("replacing string")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("search in a another path")
                .default_value("."),
        )
        .arg(
            Arg::new("test")
                .short('t')
                .long("test")
                .help("do not apply changes")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let substitution = matches.get_one::<String>("substitution").unwrap();
    let path = matches.get_one::<String>("path").unwrap();
    let test = matches.get_one::<bool>("test").unwrap();

    rename(pattern, substitution, path, test).unwrap();
}

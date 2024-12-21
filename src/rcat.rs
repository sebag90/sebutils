use clap::{Arg, Command};
use std::fs;
use std::io::{self, BufRead};
use walkdir::WalkDir;

fn search(path: &str) -> io::Result<()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok()) // skip files without permissions
        .filter(|e| e.file_type().is_file())
    // ignore folders
    {
        if let Ok(file) = fs::File::open(entry.path()) {
            let reader = io::BufReader::new(file);

            // Read lines from the file and print them.
            for line_result in reader.lines() {
                match line_result {
                    Ok(valid_line) => {
                        println!("{}", valid_line);
                    }
                    Err(_e) => break,
                }
            }
        }
    }
    Ok(())
}

fn main() {
    let matches = Command::new("rcat")
        .version("1.0")
        .about("Recursive cat")
        .arg(
            Arg::new("path")
                .help("Root path to start search")
                .index(1)
                .default_value("."), // Default value for "path"
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    search(path).unwrap();
}

extern crate clap;

use clap::{Arg, Command};
use std::fs;
use std::io::{self, BufRead};

fn search(path: &str) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            search(&path.to_string_lossy())?; // Recursive call, propagating errors
        } else {
            let file = fs::File::open(&path)?;
            let reader = io::BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(valid_line) => {
                        println!("{}", valid_line); // Print each line
                    }
                    Err(_e) => {
                        break;
                    }
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

    // Since `default_value` guarantees a value, we can safely unwrap.
    let path = matches.get_one::<String>("path").unwrap();
    search(path).unwrap(); // The `?` operator works now
}

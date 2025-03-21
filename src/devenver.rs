use clap::{Arg, Command};
use std::fs;
use std::io::{self};
use walkdir::WalkDir;

struct Colors;
impl Colors {
    const PURPLE: &'static str = "\u{001b}[95m";
    const END: &'static str = "\u{001b}[0m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn rm_ds_files(path: &str) -> io::Result<()> {
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == ".venv" {
            let color_filename = color_string(Colors::PURPLE, &entry.path().display().to_string());
            println!("{}", color_filename);
            fs::remove_dir_all(&entry.path())?;
        }
    }
    Ok(())
}

fn main() {
    let matches = Command::new("Devenver")
        .version("1.0")
        .about("Deletes .venv directory")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("root path to start recursive search")
                .default_value("."),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();

    rm_ds_files(path).unwrap();
}

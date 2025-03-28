use clap::{Arg, Command};
use std::fs;
use std::io::{self};
use walkdir::WalkDir;
extern crate fs_extra;
use fs_extra::dir::get_size;
use human_bytes::human_bytes;

struct Colors;
impl Colors {
    const PURPLE: &'static str = "\u{001b}[95m";
    const END: &'static str = "\u{001b}[0m";
    const GREEN: &'static str = "\u{001b}[92m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn rm_venv_dirs(path: &str) -> io::Result<()> {
    let mut total_used_space = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_name() == ".venv" {
            let color_filename = color_string(Colors::PURPLE, &entry.path().display().to_string());
            let folder_size = get_size(&entry.path()).unwrap();
            total_used_space = total_used_space + folder_size;
            println!("{}", color_filename);
            fs::remove_dir_all(&entry.path())?;
        }
    }
    let color_used_space = color_string(Colors::GREEN, &human_bytes(total_used_space as f64));
    println!("\nTotal reclaimed space: {}", color_used_space);
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

    rm_venv_dirs(path).unwrap();
}

use clap::{Arg, Command};
use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use rayon::prelude::*;
use std::fs;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

struct Colors;
impl Colors {
    const PURPLE: &'static str = "\u{001b}[95m";
    const GREEN: &'static str = "\u{001b}[92m";
    const END: &'static str = "\u{001b}[0m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn rm_venv_dirs_rayon(path: &str, dry_run: bool) -> io::Result<()> {
    // 1. Collect all `.venv` directories first (walk is serial)
    let venv_paths: Vec<PathBuf> = WalkDir::new(path)
        .into_iter()
        .par_bridge()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_name().to_str() == Some(".venv"))
        .map(|entry| entry.into_path())
        .collect();

    // 2. Process in parallel using rayon
    let results: Vec<(PathBuf, u64, Option<io::Error>)> = venv_paths
        .par_iter()
        .map(|venv_path| {
            let size = get_size(venv_path).unwrap_or(0);
            let err = if dry_run {
                None // no deletion attempted
            } else {
                fs::remove_dir_all(venv_path).err()
            };
            (venv_path.clone(), size, err)
        })
        .collect();

    // 3. Display results and accumulate total space saved
    let mut total_used_space = 0;
    for (path, size, err) in results {
        let color_filename = color_string(Colors::PURPLE, &path.display().to_string());
        if let Some(e) = err {
            eprintln!("Failed to delete {}: {}", color_filename, e);
        } else {
            println!("{}", color_filename);
            total_used_space += size;
        }

        total_used_space += size;
    }

    let color_used_space = color_string(Colors::GREEN, &human_bytes(total_used_space as f64));
    println!("\nTotal reclaimed space: {}", color_used_space);

    Ok(())
}

fn main() {
    let matches = Command::new("Devenver")
        .version("1.0")
        .about("Recursively deletes .venv directories")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("Root path to start recursive search")
                .default_value("."),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show which directories would be deleted without actually deleting them")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let dry_run = matches.get_flag("dry-run");

    if let Err(err) = rm_venv_dirs_rayon(path, dry_run) {
        eprintln!("Error: {}", err);
    }
}

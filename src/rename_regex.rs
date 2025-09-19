use clap::{Arg, Command};
use regex::Regex;
use std::fs::{self};
use std::path::Path;

struct Colors;
impl Colors {
    // const CYAN: &'static str = "\u{001b}[96m";
    const GREEN: &'static str = "\u{001b}[92m";
    const RED: &'static str = "\u{001b}[91m";
    const END: &'static str = "\u{001b}[0m";
    const YELLOW: &'static str = "\u{001b}[93m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn rename(pattern: &str, replace: &str, root_path: &str, dry_run: bool) -> std::io::Result<()> {
    let mut filename_mapping: Vec<(String, String)> = Vec::new();

    match Regex::new(pattern) {
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
                                println!(
                                    "{} -> {}",
                                    color_string(Colors::YELLOW, &path.display().to_string()),
                                    color_string(Colors::GREEN, &new_name)
                                );

                                if dry_run != true {
                                    if Path::new(&new_name).exists() {
                                        println!(
                                            "{}",
                                            color_string(
                                                Colors::RED,
                                                &format!("Error: '{}' already exists!", new_name)
                                            )
                                        );

                                        return Ok(());
                                    }

                                    filename_mapping.push((path.display().to_string(), new_name));
                                }
                            }
                        }
                    }
                }
            }

            // Create new files from the temporary files
            for (old_name, new_file_name) in filename_mapping.into_iter() {
                fs::rename(old_name, new_file_name)?;
            }
            Ok(())
        }
        Err(_) => {
            println!("Invalid Regex :(");
            return Ok(());
        }
    }
}

fn main() {
    let matches = Command::new("rename-regex")
        .version("1.0")
        .about("Rename all files with regex")
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
            Arg::new("dry-run")
                .long("dry-run")
                .help("SDo not apply changes")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let pattern = matches.get_one::<String>("pattern").unwrap();
    let substitution = matches.get_one::<String>("substitution").unwrap();
    let path = matches.get_one::<String>("path").unwrap();
    let dry_run = matches.get_flag("dry-run");

    rename(pattern, substitution, path, dry_run).unwrap();
}

use clap::{Arg, ArgAction, Command};
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use walkdir::DirEntry;
use walkdir::WalkDir;

struct Colors;
impl Colors {
    const PURPLE: &'static str = "\u{001b}[95m";
    const GREEN: &'static str = "\u{001b}[92m";
    const RED: &'static str = "\u{001b}[91m";
    const END: &'static str = "\u{001b}[0m";
    const YELLOW: &'static str = "\u{001b}[93m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn search_in_file(regex: Regex, filename: DirEntry, ignore_case: bool) {
    if let Ok(file) = fs::File::open(filename.path()) {
        let reader = io::BufReader::new(file);
        for (line_idx, line) in reader.lines().enumerate() {
            match line {
                Ok(mut valid_line) => {
                    if ignore_case {
                        valid_line = valid_line.to_lowercase()
                    };

                    if let Some(matched) = regex.find(&valid_line) {
                        let color_filename =
                            color_string(Colors::PURPLE, &filename.path().display().to_string());

                        let idx =
                            color_string(Colors::YELLOW, &format!("{}", &line_idx.to_string()));
                        let row_idx = color_string(Colors::GREEN, &format!("{}", matched.start()));

                        let result = format!(
                            "{}{}{}",
                            &valid_line[0..matched.start()],
                            color_string(Colors::RED, matched.as_str()),
                            &valid_line[matched.end()..]
                        )
                        .trim()
                        .to_string();

                        println!("{}:{}:{}\t{}", color_filename, idx, row_idx, result);
                    }
                }
                Err(_e) => break,
            }
        }
    }
}

fn search(
    path: &str,
    pattern: &str,
    file_name: Option<&String>,
    ignore_case: bool,
    name_only: bool,
    dirs_only: bool,
) -> io::Result<()> {
    let regex = if ignore_case {
        Regex::new(&format!("(?i){}", pattern)).unwrap()
    } else {
        Regex::new(pattern).unwrap()
    };

    let entries: Vec<_> = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .collect();
    // skip files without permissions
    entries.par_iter().for_each(|entry| {
        if name_only {
            if dirs_only {
                // ignore everything that isn't a directory
                if !entry.file_type().is_dir() {
                    return;
                }
            } else {
                // ignore directories
                if entry.file_type().is_dir() {
                    return;
                }
            }

            if let Some(_matched) = regex.find(&entry.path().display().to_string()) {
                let color_filename =
                    color_string(Colors::PURPLE, &entry.path().display().to_string());

                println!("{}", color_filename);
            }

        // we search the entire file, not just the name
        } else {
            let to_check = match file_name {
                Some(name) => {
                    let name_regex = Regex::new(name).unwrap();
                    name_regex
                        .find(&entry.path().display().to_string())
                        .is_some()
                }
                None => true,
            };

            if to_check {
                search_in_file(regex.clone(), entry.clone(), ignore_case)
            }
        }
    });
    Ok(())
}

fn main() {
    let matches = Command::new("Searcher")
        .version("1.0")
        .about("Searches for patterns in files")
        .arg(
            Arg::new("pattern")
                .help("search pattern")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("root path to start recursive search")
                .default_value("."),
        )
        .arg(
            Arg::new("file-name")
                .short('f')
                .long("file-name")
                .help("only search files that match the file name regex"),
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .action(ArgAction::SetTrue)
                .long("ignore-case")
                .help("ignore case distinctions in patterns and data"),
        )
        .arg(
            Arg::new("name-only")
                .short('n')
                .action(ArgAction::SetTrue)
                .long("name-only")
                .help("match the name only and not the content"),
        )
        .arg(
            Arg::new("dirs-only")
                .short('d')
                .action(ArgAction::SetTrue)
                .long("dirs-only")
                .help("match the name of directories only")
                .requires("name-only"),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let file_name = matches.get_one::<String>("file-name");
    let ignore_case = matches.get_one::<bool>("ignore-case").unwrap();
    let name_only = matches.get_one::<bool>("name-only").unwrap();
    let dirs_only = matches.get_one::<bool>("dirs-only").unwrap();

    // Validate main regex pattern early
    let _main_regex = if *ignore_case {
        Regex::new(&format!("(?i){}", pattern))
    } else {
        Regex::new(pattern)
    };

    let _main_regex = match _main_regex {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Invalid regex pattern: {}", e);
            std::process::exit(1);
        }
    };

    // Validate file-name regex if present
    let _file_name_regex = match file_name {
        Some(name) => match Regex::new(name) {
            Ok(r) => Some(r),
            Err(e) => {
                eprintln!("Invalid file-name regex: {}", e);
                std::process::exit(1);
            }
        },
        None => None,
    };

    search(
        path,
        pattern,
        file_name,
        *ignore_case,
        *name_only,
        *dirs_only,
    )
    .unwrap();
}

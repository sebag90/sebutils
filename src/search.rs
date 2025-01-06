use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use walkdir::WalkDir;

struct Colors;
impl Colors {
    const CYAN: &'static str = "\u{001b}[96m";
    const GREEN: &'static str = "\u{001b}[92m";
    const RED: &'static str = "\u{001b}[91m";
    const END: &'static str = "\u{001b}[0m";
    const YELLOW: &'static str = "\u{001b}[93m";
}

fn color_string(color: &str, message: &str) -> String {
    format!("{}{}{}", color, message, Colors::END)
}

fn search(path: &str, pattern: &str, ignore_case: bool) -> io::Result<()> {
    let regex = if ignore_case {
        Regex::new(&format!("(?i){}", pattern)).unwrap()
    } else {
        Regex::new(pattern).unwrap()
    };

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok()) // skip files without permissions
        .filter(|e| e.file_type().is_file())
    // ignore folders
    {
        if let Ok(file) = fs::File::open(entry.path()) {
            let reader = io::BufReader::new(file);
            for (line_idx, line) in reader.lines().enumerate() {
                match line {
                    Ok(mut valid_line) => {
                        // valid_line = valid_line.trim().to_string();

                        if ignore_case {
                            valid_line = valid_line.to_lowercase()
                        };

                        if let Some(matched) = regex.find(&valid_line) {
                            let color_filename =
                                color_string(Colors::CYAN, &entry.path().display().to_string());

                            let idx =
                                color_string(Colors::YELLOW, &format!("{}", &line_idx.to_string()));
                            let row_idx =
                                color_string(Colors::GREEN, &format!("{}", matched.start()));

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
    Ok(())
}

fn main() {
    let matches = Command::new("Searcher")
        .version("1.0")
        .about("Searches for patterns in files")
        .arg(
            Arg::new("pattern")
                .help("Search pattern")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .help("search in a another path")
                .default_value("."),
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .action(ArgAction::SetTrue)
                .long("ignore-case")
                .help("Ignore case sensitivity"),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let ignore_case = matches.get_one::<bool>("ignore-case").unwrap();

    search(path, pattern, *ignore_case).unwrap();
}

use clap::{Arg, Command};
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
                    Ok(valid_line) => {
                        let trimmed_line = valid_line.trim();
                        let search_line = if ignore_case {
                            trimmed_line.to_lowercase()
                        } else {
                            trimmed_line.to_owned()
                        };

                        if let Some(matched) = regex.find(&search_line) {
                            let color_filename =
                                color_string(Colors::CYAN, &entry.path().display().to_string());

                            let idx =
                                color_string(Colors::YELLOW, &format!("{}", &line_idx.to_string()));
                            let row_idx =
                                color_string(Colors::GREEN, &format!("{}", matched.start()));

                            let result = format!(
                                "{}{}{}",
                                &trimmed_line[0..matched.start()],
                                color_string(Colors::RED, matched.as_str()),
                                &trimmed_line[matched.end()..]
                            );

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
            Arg::new("path")
                .help("Root path to start search")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("pattern")
                .help("Search pattern")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .long("ignore-case")
                .help("Ignore case sensitivity"),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let pattern = matches.get_one::<String>("pattern").unwrap();
    let ignore_case = matches.contains_id("ignore-case");

    search(path, pattern, ignore_case).unwrap();
}

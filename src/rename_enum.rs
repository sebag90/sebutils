use clap::{Arg, Command};
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::MAIN_SEPARATOR_STR;
use std::path::{Path, PathBuf};
use tempfile::tempfile;

fn rename(root_path: &str) -> std::io::Result<()> {
    let mut tmp_files: Vec<(File, PathBuf)> = Vec::new();

    // Iterate over the files in the current directory
    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                if !stem.to_string_lossy().starts_with('.') {
                    let mut temp_file = tempfile()?;
                    let content = fs::read(&path)?;
                    temp_file.write_all(&content)?;
                    temp_file.seek(SeekFrom::Start(0)).unwrap();
                    tmp_files.push((temp_file, path.clone()));
                    fs::remove_file(&path)?;
                }
            }
        }
    }

    // Create new files from the temporary files
    for (index, (mut tmp_file, original_path)) in tmp_files.into_iter().enumerate() {
        let suffix = original_path
            .extension()
            .map_or("".to_string(), |ext| format!(".{}", ext.to_string_lossy()));

        let base_file_name = format!("{}{}{}", root_path, MAIN_SEPARATOR_STR, index);

        let new_file_name = format!("{}{}", base_file_name, suffix);
        let new_path = Path::new(&new_file_name);
        let mut new_file = File::create(&new_path)?;
        let mut buffer = Vec::new();
        tmp_file.read_to_end(&mut buffer)?;
        new_file.write_all(&buffer)?;
        println!("{} -> {}", original_path.display(), new_file_name);
    }

    Ok(())
}

fn main() {
    let matches = Command::new("rename-enum")
        .version("1.0")
        .about("Rename all files by enumerating")
        .arg(
            Arg::new("path")
                .help("Root path to process files")
                .index(1)
                .default_value("."), // Default value for "path"
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    rename(path).unwrap();
}

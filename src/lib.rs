use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};


pub fn run() -> Result<(), Box<dyn Error>> {
    let extensions = [
        "rs", "toml", "html", "css", "js", "ts", "svelte",
        "c", "cpp", "cs", "bash", "java", "json", "py", "ipynb",
        "md",
    ];

    let ignored_directories = [
        ".git", "__venv__", "node_modules", "target", "__pycache__", ".svelte-kit",
        ".next", "build", ".expo", ".idea"
    ];

    let ignored_files = [
        "package-lock.json"
    ];

    let mut composition_hashmap: HashMap<&str, usize> = extensions.iter()
        .map(|&ext| (ext, 0))
        .collect();

    let entries: Vec<_> = WalkDir::new(".")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            !entry.path().ancestors().any(|ancestor| {
                ignored_directories.contains(&ancestor.file_name().unwrap_or_default().to_str().unwrap_or_default())
            }) &&
            !ignored_files.contains(&entry.path().file_name().unwrap_or_default().to_str().unwrap_or_default())
        })
        .collect();

    let total_entries = entries.len();

    let progress_bar = ProgressBar::new(total_entries as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .expect("Invalid progress bar template")
        .progress_chars("#>-"));

    for entry in entries {
        let path = entry.path();
        progress_bar.inc(1);

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Some(line_count) = composition_hashmap.get_mut(ext.to_string().as_str()) {
                    *line_count += count_lines_in_file(path)?;
                }
            }
        }
    }

    progress_bar.finish_and_clear();

    for (ext, count) in composition_hashmap {
        if count != 0 {
            println!("{}: {} lines", ext, count);
        }
    }

    Ok(())
}

fn count_lines_in_file(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().count())
}
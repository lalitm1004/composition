use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path
};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::{DirEntry, WalkDir};

pub fn run() -> Result<(), Box<dyn Error>> {
    
    let entries = get_entries(".")?;

    let composition_hashmap = count_lines(entries)?;

    display_composition(composition_hashmap);
    Ok(())
}

fn display_composition(composition_hashmap: HashMap<String, usize>) {
    let total_lines: usize = composition_hashmap.values().sum();

    let mut sorted_entries: Vec<(String, usize)> = composition_hashmap.into_iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));

    for (ext, line_count) in sorted_entries {
        if line_count != 0 {
            let percentage: f64 = (line_count as f64 / total_lines as f64) * 100.0;
            println!("{} : {} lines [{:.2}%]", ext, line_count ,percentage);
        }
    } 
}

fn count_lines(entries: Vec<DirEntry>) -> Result<HashMap<String, usize>, Box<dyn Error>> {
    let extensions: HashSet<&str> = [
        "rs", "toml", "html", "css", "js", "ts", "svelte",
        "c", "cpp", "cs", "bash", "java", "py", "ipynb",
        "md",
    ].into();

    let mut composition_hashmap = HashMap::new();
    composition_hashmap.extend(extensions.iter().map(|&ext| (ext.to_string(), 0)));

    let progress_bar = ProgressBar::new(entries.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:..green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")?
            .progress_chars("#>-"),  
    );

    for entry in entries {
        let path = entry.path();
        progress_bar.inc(1);

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                if let Some(line_count) = composition_hashmap.get_mut(ext) {
                    *line_count += get_lines_in_file(path)?;
                }
            }
        }
    }

    progress_bar.finish_and_clear();
    Ok(composition_hashmap)
}

fn get_entries(root: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let ignored_directories: HashSet<&str> = [
        ".git", "__venv__", "node_modules", "target", "__pycache__", ".svelte-kit",
        ".next", "build", ".expo", ".idea", "venv",
    ].into();

    let ignored_files: HashSet<&str> = [
        "package-lock.json"
    ].into();

    let entries: Vec<DirEntry> = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            !entry.path().ancestors().any(|ancestor| {
                ignored_directories.contains(ancestor.file_name().unwrap_or_default().to_str().unwrap_or_default())
            }) &&
            !ignored_files.contains(entry.path().file_name().unwrap_or_default().to_str().unwrap_or_default())
        })
        .collect();

    Ok(entries)
}

fn get_lines_in_file(path: &Path) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines().count())
}
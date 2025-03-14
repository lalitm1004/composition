use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
    path::{Path, PathBuf},
};

use walkdir::{DirEntry, WalkDir};

use crate::{
    Tracked,
    settings::{should_ignore_directory, should_ignore_file, should_ignore_line},
};

pub fn get_composition(
    root: &PathBuf,
    tracked_extensions: &Vec<Tracked>,
) -> Result<HashMap<String, usize>, io::Error> {
    let mut composition_hashmap: HashMap<String, usize> = HashMap::new();

    let all_entries = get_all_entires(root);
    for entry in all_entries {
        let path = entry.path();
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        for tracked in tracked_extensions {
            if tracked.extensions.contains(&extension) {
                let count = composition_hashmap
                    .entry(tracked.display.clone())
                    .or_insert(0);
                *count += get_line_count(path)?;
            }
        }
    }

    Ok(composition_hashmap)
}

pub fn get_investigation(
    root: &PathBuf,
    tracked_extension: Tracked,
) -> Result<HashMap<String, usize>, io::Error> {
    let mut investigation_hashmap: HashMap<String, usize> = HashMap::new();
    investigation_hashmap.insert("./".to_string(), 0);

    let root_path = root.canonicalize()?;

    let all_entries = get_all_entires(&root_path);

    for entry in all_entries {
        let path = entry.path();

        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext.to_string(),
            None => continue,
        };

        if !tracked_extension.extensions.contains(&extension) {
            continue;
        }

        let parent = match path.parent() {
            Some(p) => p.canonicalize()?,
            None => continue,
        };

        let dir_key = if parent == root_path {
            ".".to_string()
        } else {
            let mut depth_1_dir = ".".to_string();

            if let Some(parent_parent) = parent.parent() {
                if parent_parent == root_path {
                    if let Some(dir_name) = parent.file_name() {
                        if let Some(name_str) = dir_name.to_str() {
                            depth_1_dir = name_str.to_string();
                        }
                    }
                } else {
                    let parent_str = parent.to_string_lossy();
                    let root_str = root_path.to_string_lossy();

                    if parent_str.starts_with(&*root_str) {
                        let relative = &parent_str[root_str.len()..];
                        let relative = relative.trim_start_matches(std::path::MAIN_SEPARATOR);

                        if let Some(first_dir) = relative.split(std::path::MAIN_SEPARATOR).next() {
                            if !first_dir.is_empty() {
                                depth_1_dir = first_dir.to_string();
                            }
                        }
                    }
                }
            }

            depth_1_dir
        };

        let count = investigation_hashmap
            .entry(format!("{}/", dir_key))
            .or_insert(0);
        *count += get_line_count(path)?;
    }

    Ok(investigation_hashmap)
}

fn get_line_count(file_path: &Path) -> Result<usize, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let line_count = reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !should_ignore_line(line))
        .count();

    Ok(line_count)
}

pub fn get_all_entires(path: &PathBuf) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| {
            let file_name = e.file_name().to_str().unwrap();
            !should_ignore_directory(file_name)
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file()
                && !should_ignore_file(e.path().file_name().unwrap().to_str().unwrap())
        })
        .collect()
}

use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{self, BufRead}
};
use walkdir::{DirEntry, WalkDir};
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;

use config::Tracked;
pub mod config;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>>{
    let directory_entries = get_directory_entries(&config);

    let composition_hashmap = get_composition(directory_entries)?;

    display_composition(composition_hashmap, &config);
    Ok(())
}

fn get_directory_entries(config: &config::Config) -> Vec<DirEntry> {
    let ignored_directories = config::get_ignored_directories();
    let ignored_files = config::get_ignored_files();

    let entries: Vec<DirEntry> = WalkDir::new(&config.root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| {
            let path = entry.path();

            !path.ancestors().any(|ancestor| {
                ignored_directories.contains(ancestor.file_name().unwrap_or_default().to_str().unwrap_or_default())
            }) &&
            !ignored_files.contains(path.file_name().unwrap_or_default().to_str().unwrap_or_default())
        })
        .collect();

    entries
}

fn get_composition(directory_entries: Vec<DirEntry>) -> Result<HashMap<&'static Tracked, usize>, Box<dyn Error>> {
    let mut composition_hashmap: HashMap<&Tracked, usize> = config::get_tracked_extensions()
        .into_iter()
        .map(|tracked| (tracked, 0))
        .collect();

    let progress_bar = ProgressBar::new(directory_entries.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:..green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")?
            .progress_chars("#>-"),
    );

    for entry in directory_entries {
        let path = entry.path();
        progress_bar.inc(1);

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                if let Some((_tracked, line_count)) = composition_hashmap.iter_mut().find(|(&tracked , _)| tracked.ext == ext.to_string()) {
                    let file = File::open(path)?;
                    let reader = io::BufReader::new(file);

                    *line_count += reader.lines().count()
                }
            }
        }
    }

    progress_bar.finish_and_clear();
    Ok(composition_hashmap)
}

fn display_composition(composition_hashmap: HashMap<&'static Tracked, usize>, config: &config::Config) {
    let total_lines: usize = composition_hashmap.values().sum();

    let mut sorted_entries: Vec<(&Tracked, usize, f32)> = composition_hashmap
        .into_iter()
        .map(|entry| {
            let percentage: f32 = (entry.1 as f32) / (total_lines as f32) * 100.0;
            (entry.0, entry.1, percentage)
        })
        .collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));

    for (tracked, line_count, percentage) in sorted_entries {
        if line_count == 0 {
            continue;
        }

        let bar_width = (percentage / &config.minify).round() as usize;

        let mut bar = "█".repeat(bar_width);

        if *config::get_colored_composition_bar() {
            let color = Color::TrueColor {
                r: tracked.color.0,
                g: tracked.color.1,
                b: tracked.color.2,
            };
            bar = bar.color(color).to_string();
        }

        println!("{:<10} | {:>10} lines | {:>5.2}% | {}", tracked.display, line_count, percentage, bar);
    }
}
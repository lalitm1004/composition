use rayon::prelude::*;
use std::{collections::HashMap, fs};
use walkdir::DirEntry;

use crate::context::{AppContext, config::Tracked};

#[derive(Debug)]
pub struct CompositionEntry {
    pub tracked: Tracked,
    pub line_count: usize,
}

pub fn get_composition(app_context: &AppContext, entries: Vec<DirEntry>) -> Vec<CompositionEntry> {
    // Process all files in parallel and count lines per extension
    let line_counts_by_extension: HashMap<String, usize> = entries
        .par_iter()
        .filter_map(|entry| {
            let ext = entry.path().extension()?.to_str()?.to_lowercase();
            let lines = count_lines(entry.path(), app_context, &ext)?;
            Some((ext, lines))
        })
        .fold(
            || HashMap::new(),
            |mut map, (ext, lines)| {
                *map.entry(ext).or_insert(0) += lines;
                map
            },
        )
        .reduce(
            || HashMap::new(),
            |mut map1, map2| {
                for (ext, lines) in map2 {
                    *map1.entry(ext).or_insert(0) += lines;
                }
                map1
            },
        );

    // Map the line counts to tracked languages
    let composition: Vec<CompositionEntry> = app_context
        .config
        .tracked
        .iter()
        .filter_map(|tracked| {
            let total_lines: usize = tracked
                .extensions
                .iter()
                .filter_map(|ext| line_counts_by_extension.get(ext))
                .sum();

            if total_lines == 0 {
                return None;
            }

            Some(CompositionEntry {
                tracked: Tracked {
                    display: tracked.display.clone(),
                    extensions: tracked.extensions.clone(),
                    color: tracked.color.clone(),
                    excluded_patterns: tracked.excluded_patterns.clone(),
                    compiled_excluded_patterns: tracked.compiled_excluded_patterns.clone(),
                },
                line_count: total_lines,
            })
        })
        .collect();

    composition
}

fn count_lines(path: &std::path::Path, app_context: &AppContext, ext: &str) -> Option<usize> {
    let content = fs::read_to_string(path).ok()?;

    // Find the tracked config for this extension to get language-specific patterns
    let tracked = app_context
        .config
        .tracked
        .iter()
        .find(|t| t.extensions.iter().any(|e| e == ext))?;

    let count = content
        .lines()
        .filter(|line| {
            // Check if we should ignore empty lines
            if app_context.config.ignore_empty_lines && line.trim().is_empty() {
                return false;
            }

            // Check global excluded patterns
            for pattern in &app_context.config.compiled_excluded_patterns {
                if pattern.is_match(line) {
                    return false;
                }
            }

            // Check language-specific excluded patterns
            for pattern in &tracked.compiled_excluded_patterns {
                if pattern.is_match(line) {
                    return false;
                }
            }

            true
        })
        .count();

    Some(count)
}

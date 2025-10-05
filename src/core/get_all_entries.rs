use ignore::gitignore::{Gitignore, GitignoreBuilder};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

use crate::context::AppContext;

pub fn get_all_entries(app_context: &AppContext) -> Vec<DirEntry> {
    let tracked_extensions: HashSet<String> = app_context
        .config
        .tracked
        .iter()
        .flat_map(|t| t.extensions.iter().map(|e| e.to_lowercase()))
        .collect();

    let ignored_files: HashSet<String> = app_context.config.ignored_files.iter().cloned().collect();

    let ignored_directories: HashSet<String> = app_context
        .config
        .ignored_directories
        .iter()
        .cloned()
        .collect();

    // canonicalize root once
    let canonical_root = app_context
        .path
        .canonicalize()
        .unwrap_or_else(|_| app_context.path.clone());

    // build gitignore cache first
    let gitignore_cache = if app_context.config.respect_gitignore {
        build_gitignore_cache(
            &app_context.path,
            &canonical_root,
            app_context.config.ignore_dotfolders,
            &ignored_directories,
        )
    } else {
        HashMap::new()
    };

    // collect all entries (sequential)
    let all_entries = collect_directory_entries(
        &app_context.path,
        app_context.config.ignore_dotfolders,
        &ignored_directories,
    );

    // filter entries in parallel
    filter_entries_parallel(
        all_entries,
        app_context,
        &canonical_root,
        &tracked_extensions,
        &ignored_files,
        &gitignore_cache,
    )
}

fn should_traverse_directory(
    entry: &DirEntry,
    root: &Path,
    ignore_dotfolders: bool,
    ignored_directories: &HashSet<String>,
) -> bool {
    // check directory ignore settings
    if !entry.file_type().is_dir() {
        return true;
    }

    if entry.path() == root {
        return true;
    }

    let file_name = entry.file_name().to_string_lossy();

    if ignore_dotfolders && file_name.starts_with('.') {
        return false;
    }

    if ignored_directories.contains(file_name.as_ref()) {
        return false;
    }

    true
}

fn collect_directory_entries(
    root: &Path,
    ignore_dotfolders: bool,
    ignored_directories: &HashSet<String>,
) -> Vec<DirEntry> {
    WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            should_traverse_directory(e, root, ignore_dotfolders, ignored_directories)
        })
        .filter_map(Result::ok)
        .collect()
}

fn filter_entries_parallel(
    entries: Vec<DirEntry>,
    app_context: &AppContext,
    canonical_root: &Path,
    tracked_extensions: &HashSet<String>,
    ignored_files: &HashSet<String>,
    gitignore_cache: &HashMap<PathBuf, Gitignore>,
) -> Vec<DirEntry> {
    entries
        .into_par_iter()
        .filter(|entry| {
            entry.file_type().is_file()
                && filter_file(
                    entry,
                    app_context,
                    canonical_root,
                    tracked_extensions,
                    ignored_files,
                    gitignore_cache,
                )
        })
        .collect()
}

fn build_gitignore_cache(
    root: &Path,
    canonical_root: &Path,
    ignore_dotfolders: bool,
    ignored_directories: &HashSet<String>,
) -> HashMap<PathBuf, Gitignore> {
    let mut cache = HashMap::new();

    // root gitignore
    if let Ok(gitignore) = build_gitignore(root) {
        cache.insert(canonical_root.to_path_buf(), gitignore);
    }

    // walk and build cache for all directories
    let directories = collect_directory_entries(root, ignore_dotfolders, ignored_directories)
        .into_iter()
        .filter(|e| e.file_type().is_dir() && e.path() != root);

    for dir_entry in directories {
        // infer canonical path by appending relative path to canonical root
        if let Some(canonical_dir) = infer_canonical_path(dir_entry.path(), root, canonical_root) {
            if let Ok(gitignore) = build_gitignore(dir_entry.path()) {
                cache.insert(canonical_dir, gitignore);
            }
        }
    }

    cache
}

fn infer_canonical_path(path: &Path, root: &Path, canonical_root: &Path) -> Option<PathBuf> {
    path.strip_prefix(root)
        .ok()
        .map(|relative| canonical_root.join(relative))
}

fn filter_file(
    entry: &DirEntry,
    app_context: &AppContext,
    canonical_root: &Path,
    tracked_extensions: &HashSet<String>,
    ignored_files: &HashSet<String>,
    gitignore_cache: &HashMap<PathBuf, Gitignore>,
) -> bool {
    let file_name = entry.file_name().to_string_lossy();

    if app_context.config.ignore_dotfiles && file_name.starts_with('.') {
        return false;
    }

    if ignored_files.contains(file_name.as_ref()) {
        return false;
    }

    // check extension
    if !has_tracked_extension(entry.path(), tracked_extensions) {
        return false;
    }

    // check gitignore
    if app_context.config.respect_gitignore {
        if is_ignored_by_git(entry, &app_context.path, canonical_root, gitignore_cache) {
            return false;
        }
    }

    true
}

fn has_tracked_extension(path: &Path, tracked_extensions: &HashSet<String>) -> bool {
    path.extension()
        .map(|ext| tracked_extensions.contains(&ext.to_string_lossy().to_lowercase()))
        .unwrap_or(false)
}

fn is_ignored_by_git(
    entry: &DirEntry,
    root: &Path,
    canonical_root: &Path,
    gitignore_cache: &HashMap<PathBuf, Gitignore>,
) -> bool {
    // infer canonical_path
    let canonical_path = match infer_canonical_path(entry.path(), root, canonical_root) {
        Some(path) => path,
        None => return false,
    };

    let is_dir = entry.file_type().is_dir();

    // for files, check gitignores up to and including the parent directory
    // for directories, check up to the directory itself
    let check_path = if is_dir {
        &canonical_path
    } else {
        canonical_path.parent().unwrap_or(&canonical_path)
    };

    let gitignore_stack = build_gitignore_stack(check_path, canonical_root, gitignore_cache);
    apply_gitignore_rules(&canonical_path, is_dir, &gitignore_stack)
}

fn build_gitignore_stack<'a>(
    check_path: &'a Path,
    canonical_root: &Path,
    gitignore_cache: &'a HashMap<PathBuf, Gitignore>,
) -> Vec<(&'a Path, &'a Gitignore)> {
    let mut stack = Vec::new();

    for ancestor in check_path.ancestors() {
        // stop if above root
        if !ancestor.starts_with(canonical_root) {
            break;
        }

        if let Some(gitignore) = gitignore_cache.get(ancestor) {
            stack.push((ancestor.as_ref(), gitignore));
        }
    }

    // reverse to check from parent to child
    stack.reverse();
    stack
}

fn apply_gitignore_rules(
    canonical_path: &Path,
    is_dir: bool,
    gitignore_stack: &[(&Path, &Gitignore)],
) -> bool {
    let mut final_decision = None;

    for (gitignore_dir, gitignore) in gitignore_stack {
        // calculate path relative to the gitignore directory
        let relative_path = match canonical_path.strip_prefix(gitignore_dir) {
            Ok(rel) => rel,
            Err(_) => continue,
        };

        let matched = gitignore.matched_path_or_any_parents(relative_path, is_dir);

        if matched.is_ignore() {
            final_decision = Some(true);
        } else if matched.is_whitelist() {
            final_decision = Some(false);
        }
    }

    final_decision.unwrap_or(false)
}

fn build_gitignore(dir: &Path) -> Result<Gitignore, ignore::Error> {
    let mut builder = GitignoreBuilder::new(dir);
    let gitignore_path = dir.join(".gitignore");

    if gitignore_path.exists() {
        builder.add(gitignore_path);
    }

    builder.build()
}

use ignore::gitignore::{Gitignore, GitignoreBuilder};
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

    let mut gitignore_cache: HashMap<PathBuf, Gitignore> = HashMap::new();

    // preload root gitignore with canonical path as key
    if app_context.config.respect_gitignore {
        let canonical_root = app_context
            .path
            .canonicalize()
            .unwrap_or_else(|_| app_context.path.clone());

        if let Ok(gitignore) = build_gitignore(&canonical_root) {
            gitignore_cache.insert(canonical_root.clone(), gitignore);
        }
    }

    WalkDir::new(&app_context.path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            filter_entry(
                e,
                app_context,
                &tracked_extensions,
                &ignored_directories,
                &ignored_files,
                &mut gitignore_cache,
            )
        })
        .filter_map(Result::ok)
        .filter(|dir_entry| dir_entry.file_type().is_file())
        .collect()
}

fn filter_entry(
    entry: &DirEntry,
    app_context: &AppContext,
    tracked_extensions: &HashSet<String>,
    ignored_directories: &HashSet<String>,
    ignored_files: &HashSet<String>,
    gitignore_cache: &mut HashMap<PathBuf, Gitignore>,
) -> bool {
    let path = entry.path();
    let file_name = entry.file_name().to_string_lossy();

    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let canonical_root = app_context
        .path
        .canonicalize()
        .unwrap_or_else(|_| app_context.path.clone());

    // always allow root dir
    if canonical_path == canonical_root {
        return true;
    }

    // handle dirs
    if entry.file_type().is_dir() {
        if app_context.config.ignore_dotfolders && file_name.starts_with('.') {
            return false;
        }

        if ignored_directories.contains(file_name.as_ref()) {
            return false;
        }

        // check if directory is ignored by git
        if app_context.config.respect_gitignore {
            if is_ignored_by_git(entry, &canonical_root, gitignore_cache) {
                return false;
            }
        }

        // load gitignore for this directory
        if app_context.config.respect_gitignore && !gitignore_cache.contains_key(&canonical_path) {
            if let Ok(gitignore) = build_gitignore(&canonical_path) {
                gitignore_cache.insert(canonical_path.clone(), gitignore);
            }
        }

        // allow directory traversal
        return true;
    }

    // handle files
    if entry.file_type().is_file() {
        if app_context.config.ignore_dotfiles && file_name.starts_with('.') {
            return false;
        }

        if ignored_files.contains(file_name.as_ref()) {
            return false;
        }

        // check extension
        let has_tracked_extension = match path.extension() {
            Some(ext) => {
                let ext_lower = ext.to_string_lossy().to_lowercase();
                tracked_extensions.contains(&ext_lower)
            }
            None => false,
        };

        if !has_tracked_extension {
            return false;
        }

        // check gitignore
        if app_context.config.respect_gitignore {
            // make sure parent directory's gitignore is loaded
            if let Some(parent) = canonical_path.parent() {
                let canonical_parent = parent.to_path_buf();
                if !gitignore_cache.contains_key(&canonical_parent) {
                    if let Ok(gitignore) = build_gitignore(&canonical_parent) {
                        gitignore_cache.insert(canonical_parent, gitignore);
                    }
                }
            }

            if is_ignored_by_git(entry, &canonical_root, gitignore_cache) {
                return false;
            }
        }

        return true;
    }

    // Not a regular file or directory (symlink, etc.)
    false
}

fn is_ignored_by_git(
    entry: &DirEntry,
    canonical_root: &Path,
    gitignore_cache: &HashMap<PathBuf, Gitignore>,
) -> bool {
    let path = entry.path();
    let is_dir = entry.file_type().is_dir();

    // canonicalize the entry path
    let canonical_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return false, // ignore if cant
    };

    // collect gitignored from root to current path
    let mut gitignore_stack: Vec<(&Path, &Gitignore)> = Vec::new();

    // for files, we need to check gitignores up to and including the parent directory
    // for directories, we check up to the directory itself
    let check_path = if is_dir {
        &canonical_path
    } else {
        canonical_path.parent().unwrap_or(&canonical_path)
    };

    for ancestor in check_path.ancestors() {
        // stop if above root
        if !ancestor.starts_with(canonical_root) {
            break;
        }

        if let Some(gitignore) = gitignore_cache.get(ancestor) {
            gitignore_stack.push((ancestor, gitignore));
        }
    }

    // reverse to check from parent to child
    gitignore_stack.reverse();

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

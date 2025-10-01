use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

use crate::context::AppContext;

// TODO: fix respect_gitignore

pub fn get_all_entries(app_context: &AppContext) -> Vec<DirEntry> {
    let tracked_extensions: HashSet<String> = app_context
        .config
        .tracked
        .iter()
        .flat_map(|t| t.extensions.iter().cloned())
        .collect();

    println!("{:?}", tracked_extensions);
    println!("{:?}", tracked_extensions.contains("toml"));

    let ignored_files: HashSet<String> = app_context.config.ignored_files.iter().cloned().collect();

    let ignored_directories: HashSet<String> = app_context
        .config
        .ignored_directories
        .iter()
        .cloned()
        .collect();

    let mut gitignore_cache: HashMap<PathBuf, Gitignore> = HashMap::new();

    if app_context.config.respect_gitignore {
        if let Ok(gitignore) = build_gitignore(&app_context.path) {
            gitignore_cache.insert(app_context.path.clone(), gitignore);
        }
    }

    WalkDir::new(&app_context.path)
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

    // skip root dir
    if path == app_context.path {
        return true;
    }

    // check ignored directories
    if entry.file_type().is_dir() {
        if app_context.config.ignore_dotfolders && file_name.starts_with(".") {
            return false;
        }

        if ignored_directories.contains(file_name.as_ref()) {
            return false;
        }
    }

    // check ignored files
    if entry.file_type().is_file() {
        if app_context.config.ignore_dotfiles && file_name.starts_with(".") {
            return false;
        }

        if ignored_files.contains(file_name.as_ref()) {
            return false;
        }

        if let Some(ext) = path.extension() {
            if tracked_extensions.contains(&ext.to_string_lossy().to_lowercase()) {
                return true;
            }
        }

        // early extension check
        // match path.extension() {
        //     Some(ext) if tracked_extensions.contains(&ext.to_string_lossy().to_lowercase()) => {}
        //     _ => return false,
        // }
        if let Some(ext) = path.extension() {
            if !tracked_extensions.contains(&ext.to_string_lossy().to_lowercase()) {
                return false;
            }
        } else {
            return false;
        }
    }

    if app_context.config.respect_gitignore {
        if entry.file_type().is_dir() {
            if !gitignore_cache.contains_key(path) {
                if let Ok(gitignore) = build_gitignore(path) {
                    gitignore_cache.insert(path.to_path_buf(), gitignore);
                }
            }
        }

        // check against all parent gitignores (from closest to farthest)
        // collect all applicable gitignores
        let mut applicable_gitignores: Vec<(PathBuf, &Gitignore)> = Vec::new();

        for ancestor in path.ancestors() {
            if ancestor == app_context.path.parent().unwrap_or(Path::new("")) {
                break;
            }

            if let Some(gitignore) = gitignore_cache.get(ancestor) {
                applicable_gitignores.push((ancestor.to_path_buf(), gitignore));
            }
        }

        // check gitignores from closest (lower level) to farthest (higher level)
        // lower level has higher priority
        for (gitignore_dir, gitignore) in applicable_gitignores {
            let relative_path = path.strip_prefix(&gitignore_dir).unwrap_or(path);

            let matched =
                gitignore.matched_path_or_any_parents(relative_path, entry.file_type().is_dir());

            if matched.is_ignore() {
                return false;
            } else if matched.is_whitelist() {
                // whitelisted by lower level gitignore - allow it
                return true;
            }
            // if no match, continue checking parent gitignores
        }
    }

    true
}

fn build_gitignore(dir: &Path) -> Result<Gitignore, ignore::Error> {
    let mut builder = GitignoreBuilder::new(dir);
    let gitignore_path = dir.join(".gitignore");

    if gitignore_path.exists() {
        builder.add(gitignore_path);
    }

    builder.build()
}

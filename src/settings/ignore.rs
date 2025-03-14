// ignore all folders that start with a '.' by default
const IGNORE_DOTFOLDERS: bool = true;
const IGNORED_DIRECTORIES: &[&str] = &[
    "node_modules",
    "target",
    ".git",
    ".venv",
    "__venv__",
    "__pycache__",
    //append more
];

// ignore all files that start with a '.' by default
const IGNORE_DOTFILES: bool = true;
const IGNORED_FILES: &[&str] = &[".env"];

const IGNORE_EMPTY_LINES: bool = true;
const EXCLUDED_PATTERNS: &[&str] = &["{", "}", "(", ")", "[", "]"];

pub fn should_ignore_directory(dir_name: &str) -> bool {
    IGNORED_DIRECTORIES.contains(&dir_name)
        || (IGNORE_DOTFOLDERS && dir_name.starts_with('.') && dir_name.len() > 1)
}

pub fn should_ignore_file(file_name: &str) -> bool {
    IGNORED_FILES.contains(&file_name)
        || (IGNORE_DOTFILES && file_name.starts_with('.') && file_name.len() > 1)
}

pub fn should_ignore_line(line: &str) -> bool {
    let trimmed = line.trim();
    IGNORE_EMPTY_LINES && EXCLUDED_PATTERNS.contains(&trimmed)
}

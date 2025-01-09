pub fn get_ignored_directories() -> Vec<&'static str> {
    vec![
        ".git", ".github", "target", ".venv", "venv", "__venv__",
        "__pycache__", "node_modules", ".next", ".svelte-kit", ".expo",
        // append more
    ]
}

pub fn get_ignored_files() -> Vec<&'static str> {
    vec![
        "package-lock.json", "cargo.lock",
        // append more
    ]
}
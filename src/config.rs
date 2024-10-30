use std::collections::HashSet;
use once_cell::sync::Lazy;

pub struct Config {
    pub root: String,
    pub minify: f32,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut root = String::from(".");
        let mut minify: f32 = 1.0;

        let mut args_iter = args.iter().skip(1);

        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "--minify" => {
                    if let Some(value) = args_iter.next() {
                        minify = value.parse::<f32>().map_err(|_| "failed to parse minify as f32")?;
                    } else {
                        return Err("expected a value after --minify");
                    }
                }
                other => {
                    root = other.to_string();
                }
            }
        }

        Ok(Config { root, minify })
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct Tracked {
    pub ext: &'static str,
    pub display: &'static str,
    pub color: &'static (u8, u8, u8),
}

static TRACKED_EXTENSIONS: Lazy<Vec<Tracked>> = Lazy::new(|| {
    vec![
        Tracked { ext: "rs", display: "Rust", color: &(246, 82, 9) },
        Tracked { ext: "html", display: "HTML", color: &(241, 106, 48) },
        Tracked { ext: "css", display: "CSS", color: &(48, 106, 241) },
        Tracked { ext: "js", display: "JavaScript", color: &(255, 222, 35) },
        Tracked { ext: "ts", display: "TypeScript", color: &(55, 124, 200) },
        Tracked { ext: "svelte", display: "Svelte", color: &(247, 60, 0) },
        Tracked { ext: "c", display: "C", color: &(8, 74, 134) },
        Tracked { ext: "cpp", display: "C++", color: &(8, 94, 159) },
        Tracked { ext: "cs", display: "C#", color: &(63, 8, 147) },
        Tracked { ext: "bash", display: "Bash", color: &(8, 118, 8) },
        Tracked { ext: "java", display: "Java", color: &(11, 119, 151) },
        Tracked { ext: "py", display: "Python", color: &(55, 114, 163) },
        Tracked { ext: "ipynb", display: "Jupyter", color: &(244, 119 , 39) },
        Tracked { ext: "asm", display: "Assembly", color: &(9, 51, 50) },
        Tracked { ext: "go", display: "Go", color: &(52, 190, 178) },
    ]
});

static IGNORED_DIRECTORIES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        ".git", "__venv__", "node_modules", "target", "__pycache__", ".svelte-kit",
        ".next", "build", ".expo", ".idea", "venv",
    ].into()
});

static IGNORED_FILES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "package-lock.json",
    ].into()
});

static COLORED_COMPOSITION_BAR: Lazy<bool> = Lazy::new(|| {
    true
});

pub fn get_tracked_extensions() -> &'static Vec<Tracked> {
    &TRACKED_EXTENSIONS
}

pub fn get_ignored_directories() -> &'static HashSet<&'static str> {
    &IGNORED_DIRECTORIES
}

pub fn get_ignored_files() -> &'static HashSet<&'static str> {
    &IGNORED_FILES
}

pub fn get_colored_composition_bar() -> &'static bool {
    &COLORED_COMPOSITION_BAR
}
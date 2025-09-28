use dirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, path::Path};

mod default;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default::use_color")]
    pub use_color: bool,

    #[serde(default = "default::respect_gitignore")]
    pub respect_gitignore: bool,

    #[serde(default = "default::ignore_dotfolders")]
    pub ignore_dotfolders: bool,

    #[serde(default = "default::ignored_directories")]
    pub ignored_directories: Vec<String>,

    #[serde(default = "default::ignore_dotfiles")]
    pub ignore_dotfiles: bool,

    #[serde(default = "default::ignored_files")]
    pub ignored_files: Vec<String>,

    #[serde(default = "default::ignore_empty_lines")]
    pub ignore_empty_lines: bool,

    #[serde(default = "default::excluded_patterns")]
    pub excluded_patterns: Vec<String>,

    #[serde(default = "default::tracked")]
    pub tracked: Vec<Tracked>,

    #[serde(skip)]
    pub compiled_excluded_patterns: Vec<Regex>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracked {
    pub display: String,
    pub extensions: Vec<String>,
    pub color: Option<String>,
    pub excluded_patterns: Vec<String>,

    #[serde(skip)]
    pub compiled_excluded_patterns: Vec<Regex>,
}

impl Config {
    pub fn from_config() -> (Self, bool) {
        let config_path = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("composition")
            .join("config.toml");

        match Self::load_from_path(&config_path) {
            Ok(config) => (config, true),
            Err(_) => (Self::default(), false),
        }
    }

    fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigLoadError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).map_err(|_| ConfigLoadError::FileReadFailed)?;

        let mut config: Config =
            toml::from_str(&content).map_err(|_| ConfigLoadError::TomlParseFailed)?;

        // compile all exclude regex patterns
        config.compiled_excluded_patterns = compile_regexes(&config.excluded_patterns)?;

        let mut seen_displays = HashSet::new();
        for tracked in &mut config.tracked {
            // ensure all display values are unique
            if !seen_displays.insert(tracked.display.clone()) {
                return Err(ConfigLoadError::DuplicateTrackedDisplay(
                    tracked.display.clone(),
                ));
            }

            // compile all language specific regex patterns
            tracked.compiled_excluded_patterns = compile_regexes(&tracked.excluded_patterns)?
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut config = Self {
            use_color: default::use_color(),
            respect_gitignore: default::respect_gitignore(),
            ignore_dotfolders: default::ignore_dotfolders(),
            ignored_directories: default::ignored_directories(),
            ignore_dotfiles: default::ignore_dotfiles(),
            ignored_files: default::ignored_files(),
            ignore_empty_lines: default::ignore_empty_lines(),
            excluded_patterns: default::excluded_patterns(),
            tracked: default::tracked(),
            compiled_excluded_patterns: Vec::new(),
        };

        // compile global regex patterns (ignore errors here, assume defaults are valid)
        config.compiled_excluded_patterns =
            compile_regexes(&config.excluded_patterns).unwrap_or_default();

        // compile tracked regex patterns individually
        for tracked in &mut config.tracked {
            tracked.compiled_excluded_patterns =
                compile_regexes(&tracked.excluded_patterns).unwrap_or_default();
        }

        config
    }
}

fn compile_regexes(excluded_patterns: &Vec<String>) -> Result<Vec<Regex>, ConfigLoadError> {
    let mut compiled_patterns = Vec::with_capacity(excluded_patterns.len());
    for pat in excluded_patterns {
        let regex =
            Regex::new(pat).map_err(|_| ConfigLoadError::RegexCompileFailed(pat.clone()))?;
        compiled_patterns.push(regex);
    }

    Ok(compiled_patterns)
}

#[derive(Debug)]
pub enum ConfigLoadError {
    FileReadFailed,
    TomlParseFailed,
    RegexCompileFailed(String),
    DuplicateTrackedDisplay(String),
}

impl std::error::Error for ConfigLoadError {}

impl std::fmt::Display for ConfigLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigLoadError::FileReadFailed => write!(f, "failed to read the config file"),
            ConfigLoadError::TomlParseFailed => {
                write!(f, "failed to parse the config file as TOML")
            }
            ConfigLoadError::RegexCompileFailed(pattern) => {
                write!(f, "failed to compile regex pattern: '{}'", pattern)
            }
            ConfigLoadError::DuplicateTrackedDisplay(display) => {
                write!(f, "duplicate tracked display found: '{}'", display)
            }
        }
    }
}

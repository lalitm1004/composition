pub mod cli;
pub mod config;

use regex::Regex;
use std::path::PathBuf;

use cli::Cli;
use config::Config;

#[derive(Debug)]
pub struct AppContext {
    pub path: PathBuf,
    pub scale_bar: f32,
    pub config: Config,
    pub config_loaded: bool,
}

impl AppContext {
    pub fn from_cli(cli: Cli) -> Result<Self, String> {
        let (mut config, config_loaded) = Config::from_config();

        // apply CLI overrides to config
        Self::apply_cli_overrides(&mut config, &cli)?;

        Ok(Self {
            path: cli.path,
            scale_bar: cli.scale_bar,
            config,
            config_loaded,
        })
    }

    fn apply_cli_overrides(config: &mut Config, cli: &Cli) -> Result<(), String> {
        // override boolean flags
        if let Some(use_color) = cli.use_color {
            config.use_color = use_color;
        }

        if let Some(respect_gitignore) = cli.respect_gitignore {
            config.respect_gitignore = respect_gitignore;
        }

        if let Some(ignore_dotfolders) = cli.ignore_dotfolders {
            config.ignore_dotfolders = ignore_dotfolders;
        }

        if let Some(ignore_dotfiles) = cli.ignore_dotfiles {
            config.ignore_dotfiles = ignore_dotfiles;
        }

        if let Some(ignore_empty_lines) = cli.ignore_empty_lines {
            config.ignore_empty_lines = ignore_empty_lines;
        }

        // append to existing lists (don't replace)
        if !cli.ignored_directories.is_empty() {
            config
                .ignored_directories
                .extend(cli.ignored_directories.clone());
        }

        if !cli.ignored_files.is_empty() {
            config.ignored_files.extend(cli.ignored_files.clone());
        }

        if !cli.excluded_patterns.is_empty() {
            // extend the patterns and recompile regex
            config
                .excluded_patterns
                .extend(cli.excluded_patterns.clone());
            config.compiled_excluded_patterns = Self::compile_regexes(&config.excluded_patterns)?;
        }

        Ok(())
    }

    fn compile_regexes(patterns: &[String]) -> Result<Vec<regex::Regex>, String> {
        let mut compiled = Vec::with_capacity(patterns.len());
        for pattern in patterns {
            let regex = Regex::new(pattern)
                .map_err(|_| format!("Failed to compile regex pattern: '{}'", pattern))?;
            compiled.push(regex);
        }
        Ok(compiled)
    }
}

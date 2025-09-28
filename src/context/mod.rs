pub mod cli;
pub mod config;

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
    pub fn from_cli(cli: Cli) -> Self {
        let (mut config, config_loaded) = Config::from_config();

        // apply CLI overrides to config
        Self::apply_cli_overrides(&mut config, &cli);

        AppContext {
            path: cli.path,
            scale_bar: cli.scale_bar,
            config,
            config_loaded,
        }
    }

    fn apply_cli_overrides(config: &mut Config, cli: &Cli) {
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
    }
}

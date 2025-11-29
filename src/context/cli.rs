use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(author = "lalitm1004")]
pub struct Cli {
    #[arg(
        default_value = ".",
        value_parser = parse_path,
        help = "Path to the directory of file to process"
    )]
    pub path: PathBuf,

    #[arg(
        long,
        global = true,
        default_value_t = 1.0,
        value_parser,
        value_name = "FLOAT",
        help = "Scale factor for the contribution bar"
    )]
    pub scale_bar: f32,

    #[arg(long, help = "Override use_color config setting")]
    pub use_color: Option<bool>,

    #[arg(long, short, alias = "log", action = clap::ArgAction::SetTrue, help = "Enable logarithmic scale for the bar")]
    pub log_scale: Option<bool>,

    #[arg(long, help = "Override respect_gitignore config setting")]
    pub respect_gitignore: Option<bool>,

    #[arg(long, help = "Override ignore_dotfolders config setting")]
    pub ignore_dotfolders: Option<bool>,

    #[arg(long, help = "Override ignore_dotfiles config setting")]
    pub ignore_dotfiles: Option<bool>,

    #[arg(long, help = "Override ignore_empty_lines config setting")]
    pub ignore_empty_lines: Option<bool>,
}

fn parse_path(arg: &str) -> Result<PathBuf, String> {
    let path = Path::new(arg);
    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err("does not exist".into())
    }
}

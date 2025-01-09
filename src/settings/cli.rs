use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use super::tracked::{get_tracked_extensions, Tracked};

#[derive(Parser, Debug)]
#[command(
    author = "lalitm1004",
    version = "2.0.0",
    about = "A CLI tool to understand the composition of your codebase",
    long_about = None
)]
pub struct Cli {
    #[arg(
        default_value = ".",
        value_parser = parse_path,
        help = "Specify the path to a directory"
    )]
    pub path: PathBuf,

    #[arg(
        long,
        default_value_t = 1.0,
        value_parser = parse_scale_bar,
        help = "Set the scale factor for the bars."
    )]
    pub scale_bar: f32,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Investigate {
        #[arg(
            value_parser = parse_extension
        )]
        extension: Tracked,
    },
    List
}

fn parse_path(arg: &str) -> Result<PathBuf, String> {
    let path = Path::new(arg);
    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err("is not a valid path".to_string())
    }
}

fn parse_scale_bar(arg: &str) -> Result<f32, String> {
    arg.parse::<f32>().map_err(|_| "must be a valid floating-point value".to_string())
}

fn parse_extension(arg: &str) -> Result<Tracked, String> {
    let tracked_extensions = get_tracked_extensions();
    let extension = tracked_extensions
        .iter()
        .find(|&tracked| {
            (tracked.display == arg) | tracked.extensions.contains(&arg)
        });
    
    extension
        .cloned()
        .ok_or_else(|| format!("'{}' is not a tracked extension type", arg))
}
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use super::{get_all_tracked, tracked::Tracked};

#[derive(Debug, Parser)]
#[command(author = "lalitm1004")]
pub struct Cli {
    #[arg(
        default_value=".",
        value_parser=parse_path
    )]
    pub path: PathBuf,

    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(
        long,
        global=true,
        default_value_t=1.0,
        value_parser=parse_scale_bar
    )]
    pub scale_bar: f32,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Investigate {
        #[arg(
            value_parser=parse_tracked
        )]
        tracked: Tracked,
    },
}

fn parse_path(arg: &str) -> Result<PathBuf, String> {
    let path = Path::new(arg);
    if path.exists() {
        Ok(path.to_path_buf())
    } else {
        Err(format!(
            "invalid value '{}' for [PATH]: does not exist",
            arg
        ))
    }
}

fn parse_scale_bar(arg: &str) -> Result<f32, String> {
    arg.parse::<f32>().map_err(|_| {
        format!(
            "invalid value '{}' for [SCALE_BAR]: must be a valid floating-point value",
            arg
        )
    })
}

fn parse_tracked(arg: &str) -> Result<Tracked, String> {
    let arg_lower = arg.to_lowercase();
    let all_tracked = get_all_tracked();

    all_tracked
        .into_iter()
        .find(|tracked| {
            tracked.display.to_ascii_lowercase() == arg_lower
                || tracked
                    .extensions
                    .iter()
                    .any(|ext| ext.to_ascii_lowercase() == arg_lower)
        })
        .ok_or_else(|| {
            format!(
                "invalid value '{}' for [TRACKED]: is not being tracked",
                arg
            )
        })
}

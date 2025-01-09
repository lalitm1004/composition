use std::process;

use clap::Parser;
use composition::{display::list_tracked_extensions, settings::cli::{Cli, Commands}};

fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1)
        }
    };

    println!("{:?}", cli);

    match cli.command {
        Some(Commands::List) => list_tracked_extensions(),
        Some(Commands::Investigate { extension }) => {}
        None => {}
    }
}
use std::process;

use clap::Parser;
use composition::settings::cli::{Cli, Commands};

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
        Some(Commands::List) => {},
        Some(Commands::Investigate { extension }) => {}
        None => {}
    }
}
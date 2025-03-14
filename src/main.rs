use std::process;

use clap::Parser;
use composition::{Cli, Command, run_composition, run_investigation};

fn main() {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    if let Err(err) = match &args.command {
        Some(cmd) => match cmd {
            Command::Investigate { tracked } => run_investigation(&args, tracked.clone()),
        },
        None => run_composition(&args),
    } {
        eprintln!("{err}");
        process::exit(1);
    };
}

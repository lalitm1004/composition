use std::{io, process};
use clap::Parser;
use composition::{
    core::{run_composition, run_investigation},
    display::list_tracked_extensions,
    settings::cli::{Cli, Commands},
};

fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    if let Err(err) = run_command(cli) {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run_command(cli: Cli) -> Result<(), io::Error> {
    match cli.command.clone() {
        Some(Commands::List) => {
            list_tracked_extensions();
            Ok(())
        }
        Some(Commands::Investigate { extension }) => {
            run_investigation(cli, extension)?;
            Ok(())
        }
        None => {
            run_composition(cli)?;
            Ok(())
        }
    }
}
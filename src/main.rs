mod context;
mod core;
mod display;

use clap::Parser;
use colored::Colorize;
use std::time::Instant;

use crate::{
    context::{AppContext, cli::Cli},
    core::{get_all_entries, get_composition},
    display::{display_composition, spinner},
};

fn main() {
    let cli = Cli::parse();
    let app_context = AppContext::from_cli(cli);
    if !app_context.config_loaded {
        println!(
            "{}",
            "Warning: Missing or invalid config.toml file. Using default settings."
                .yellow()
                .bold()
        );
    }

    let start = Instant::now();

    spinner::start("Walking directory...");
    let entries = get_all_entries(&app_context);
    let entries_len = entries.len();
    spinner::end();

    spinner::start("Calculating composition...");
    let mut composition = get_composition(&app_context, entries);
    spinner::end();

    let elapsed = start.elapsed();

    println!(
        "{}",
        format!(
            "Parsed {} files in {:.6} seconds",
            entries_len,
            elapsed.as_secs_f64()
        )
        .bold()
    );

    display_composition(&app_context, &mut composition);
}
